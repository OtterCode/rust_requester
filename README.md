# rust_requester
A bit of sample work showcasing oauth2 and simple data handling.

# Usage
After `cargo run`ning, it will prompt you for your google API credentials: client_id, client_secret, auth_uri, and token_uri.
Any left blank will close the program unsuccessfully, though entered values will be saved for later. To delete saved values,
delete `rust_requester.db`. The program will then walk you through an oauth2 flow in the browser. Upon completing the approval, 
please copy the final url from the browser page and paste it back into the program, it will accept either the whole url or just 
the code. It will then proceed to ruthlessly harvest your gmail labels, save them in a binary format, and print the label names.

# Credits
Free icons used in the GUI version are sourced with permission from the following:

 - [Warning icons created by Good Ware - Flaticon](https://www.flaticon.com/free-icons/warning)
 - [Cross icons created by hqrloveq - Flaticon](https://www.flaticon.com/free-icons/cross)