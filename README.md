# rust_requester
A bit of sample work showcasing oauth2 and simple data handling.

# Usage
This is a lib + dual binary project. Running `cargo run --bin rust_requester_gui --` 
will run the graphical interface, while `cargo run --bin rust_requester_cli --`
will run it on the command line. I know I could have unified it into a single application,
but the lib/bin/bin separation was intentional, to force different, more complex decisions.

After `cargo run`ning, it will prompt you for your google API credentials: client_id, client_secret, 
auth_uri, and token_uri. These can be generated at 
[the Google Developer Console.](https://console.cloud.google.com/apis/dashboard) 
The only permission it will request is email label access. Further information can be 
[found here.](https://support.google.com/googleapi/answer/6158862?hl=en)
Any left blank will close the program unsuccessfully, though entered values will be saved for later. 
Both versions share a single database file. To delete saved values, delete `rust_requester.db` or 
run the cli with the reset flag: `cargo run --bin rust_requester_cli -- -r`. The program will then 
walk you through an oauth2 flow in the browser. It will then proceed to ruthlessly harvest your gmail 
labels, save them in a binary format, and print the label names.

# Credits
Free icons used in the GUI version are sourced with permission from the following:

 - [Warning icons created by Good Ware - Flaticon](https://www.flaticon.com/free-icons/warning)
 - [Cross icons created by hqrloveq - Flaticon](https://www.flaticon.com/free-icons/cross)
