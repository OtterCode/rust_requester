use oauth2::{
    reqwest::http_client,
    ClientSecret, 
    AuthUrl, 
    TokenUrl, 
    ClientId, 
    basic::BasicClient, 
    PkceCodeChallenge, 
    CsrfToken, 
    Scope,
    AuthorizationCode, 
    RedirectUrl, 
    AccessToken, 
    TokenResponse,
};

use crate::configuration::Configuration;

pub fn gmail_label_request(config: Configuration) -> Result<String, Box<dyn std::error::Error>> {

    let token = oauth2_request(config).unwrap();

    let client = reqwest::blocking::Client::new();
    
    let res = client.get("https://www.googleapis.com/gmail/v1/users/me/labels")
        .bearer_auth(token.secret())
        .send()?
        .text()?;

    Ok(res)
}

fn oauth2_request(config: Configuration) -> Result<AccessToken, Box<dyn std::error::Error>> {
    let client = BasicClient::new(
        config.api.id.map(ClientId::new).ok_or("Missing client id")?,
        config.api.secret.map(ClientSecret::new),
        config.api.auth_url.map(AuthUrl::new).ok_or("Missing auth url")??, 
        config.api.token_url.map(TokenUrl::new).transpose()?,
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost".to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    // let response_type = oauth2::ResponseType::new("token".to_string());

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/gmail.labels".to_string()))
        .set_pkce_challenge(pkce_challenge)
        // .set_response_type(&response_type)
        // .add_extra_param("access_type", "offline")
        .url();

    println!("Browse to: {}", auth_url);


    let mut rl = rustyline::Editor::<()>::new().unwrap();

    let mut code = rl.readline("Copy the bare code from the redirect url, or the whole url here: ")?;

    if code.starts_with("http") {
        code = code
            .rsplit_once("code=")
            .unwrap()
            .1
            .split('&')
            .next()
            .unwrap()
            .to_string();
    }

    let token_result: AccessToken = client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request(http_client)?
            .access_token()
            .clone();

    Ok(token_result)
}