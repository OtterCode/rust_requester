use std::{sync::{Arc, Mutex}};
use tokio::{sync::mpsc::{self, Sender}, task::{JoinHandle, spawn}};

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::Url;

use crate::{
    configuration::Configuration,
    error::Error,
    localhost_oauth_server::{self},
};

/// The requirements of Iced mean that there are a lot of very specific and
/// sometimes unpalatable decisions I had to make while designing this lib.
/// I'm quite pleased that I was able to successfully play hot potato with the
/// terribly finicky PkceCodeVerifier and the kill signal for the TCPListener
/// in localhost_oauth_server.
/// 
/// The interface for GmailLabelRequest is less clean than I'd like, but I have
/// to put this project down at some point, it is only a portfolio piece after
/// all.
#[derive(Debug, Clone)]
pub struct GmailLabelRequest {
    oauth_client: oauth2::basic::BasicClient,
    pub oauth_auth_url: Url,
    pub pkce_verifier: Arc<Mutex<Option<PkceCodeVerifier>>>,
    pub token: Option<AccessToken>,
    kill_signal: Option<Sender<()>>,
}

impl GmailLabelRequest {
    pub async fn new(
        config: &Configuration,
    ) -> Result<GmailLabelRequest, Box<dyn std::error::Error>> {
        let (auth_url, oauth_client, pkce_verifier) = Self::oauth2_initiation(config).await?;

        Ok(Self {
            oauth_client,
            oauth_auth_url: auth_url,
            pkce_verifier: Arc::new(Mutex::new(Some(pkce_verifier))),
            token: None,
            kill_signal: None,
        })
    }

    pub fn kill(&mut self) -> Result<(), Error>{
        match self.kill_signal.as_ref() {
            Some(signal) => {
                signal.blocking_send(()).map_err(Box::from)?;
            },
            None => {}
        };
        Ok(())
    }

    async fn oauth2_initiation(
        config: &Configuration,
    ) -> Result<(Url, BasicClient, PkceCodeVerifier), Box<dyn std::error::Error>> {
        let client = BasicClient::new(
            config
                .api
                .id
                .clone()
                .map(ClientId::new)
                .ok_or("Missing client id")?,
            config.api.secret.clone().map(ClientSecret::new),
            config
                .api
                .auth_url
                .clone()
                .map(AuthUrl::new)
                .ok_or("Missing auth url")??,
            config
                .api
                .token_url
                .clone()
                .map(TokenUrl::new)
                .transpose()?,
        )
        .set_redirect_uri(RedirectUrl::new(
            "http://localhost:".to_string() + &config.local_port.unwrap_or_default().to_string(),
        )?);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/gmail.labels".to_string(),
            ))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok((auth_url, client, pkce_verifier))
    }

    pub fn oauth2_token_receiver(
        mut self,
        config: Configuration,
    ) -> (GmailLabelRequest, JoinHandle<Result<std::string::String, Error>>) {
        let (kill_sender, kill_receiver) = mpsc::channel(1);

        self.kill_signal = Some(kill_sender.clone());

        let join_handle = spawn(localhost_oauth_server::raw_tcp_listener(
            config.local_port.unwrap_or_default(),
            kill_receiver,
        ));

        (self, join_handle)
    
    }

    pub async fn oauth2_token_verification(
        mut self,
        code: String,
    ) -> Result<GmailLabelRequest, Error> {
        // This is always a bit of an intricate dance, but a Mutex Result is
        // particularly temporary, and needs immediate handling.
        let pkce_verifier = {
            self.pkce_verifier
                .try_lock()
                .map_err(|err| {
                    eprintln!("{:?}", err);
                    Error::PkceCodeVerifierLocked
                })?
                .take()
        }
        .ok_or(Error::PkceCodeVerifierMissing)?;

        let token_result: AccessToken = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(Box::from)?
            .access_token()
            .clone();

        self.token = Some(token_result);

        Ok(self)
    }

    pub async fn get_labels(self) -> Result<(String, GmailLabelRequest), Error> {
        let client = reqwest::Client::new();

        let token = self.token.as_ref().ok_or(Error::MissingToken)?;

        let res = client
            .get("https://www.googleapis.com/gmail/v1/users/me/labels")
            .bearer_auth(token.secret())
            .send()
            .await
            .map_err(Box::from)?
            .text()
            .await
            .map_err(Box::from)?;

        Ok((res, self))
    }
}
