use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    Address, Message, SmtpTransport, Transport,
};

use crate::helpers::env::env::ENV;

pub struct SmtpFunctions;

impl SmtpFunctions {
    pub fn send_email(to: &str, subject: &str, body: &str) -> Result<(), &'static str> {
        //let email
        //TODO ponerlo en el env
        let to_address_data = to.split("@").collect::<Vec<&str>>();
        let to_name = to_address_data.get(0).ok_or("error to get address name")?;
        let to_domain = to_address_data
            .get(1)
            .ok_or("error to get address domain")?;
        let to_address =
            Address::new(*to_name, *to_domain).map_err(|_| "error to construct address")?;
        let enterprise_email = ENV
            .get_string("ENTERPRISE_EMAIL")
            .map_err(|_| "no environment variable")?;
        let enterprise_password = ENV
            .get_string("ENTERPRISE_PASSWORD")
            .map_err(|_| "no environment variable")?;
        let address_data = enterprise_email.split("@").collect::<Vec<&str>>();
        let name = address_data.get(0).ok_or("error to get address name")?;
        let domain = address_data.get(1).ok_or("error to get address domain")?;
        let address = Address::new(*name, *domain).map_err(|_| "error to construct address")?;
        let email = Message::builder()
            .from(Mailbox::new(None, address))
            .to(Mailbox::new(None, to_address))
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(body))
            .map_err(|_| "error in message")?;
        let creds = Credentials::new((*name).to_string(), enterprise_password);
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .port(465)
            .credentials(creds)
            .tls(Tls::Wrapper(TlsParameters::builder("smtp.gmail.com".to_string()).build().map_err(|_| "error on tls")?))
            .build();
        match mailer.send(&email) {
            Ok(_) => return Ok(()),
            Err(_err) => {
                return Err("error sending email");
            }
        }
    }
}
