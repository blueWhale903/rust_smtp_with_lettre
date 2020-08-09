extern crate lettre;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


fn main() {

    let smtp_server: &str = "smtp.gmail.com";
    // let port: u16 = 587;

    let smtp_mail: String = format!("hglong903@gmail.com");
    let smtp_pw: String = format!("Long992003");

    let mut to_email = "mojavdarren@gmail.com";

    let email = Message::builder()
        .from(smtp_mail.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Rust SMTP test")
        .body("Hi, this mail was sent with Rust")
        .unwrap();
    
    let creds = Credentials::new(smtp_mail, smtp_pw);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(smtp_server)
    .unwrap()
    .credentials(creds)
    .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

}
