extern crate lettre;
extern crate lettre_email;

use lettre::{SmtpTransport, SmtpClient, smtp::authentication::Credentials, Transport};
use lettre_email::EmailBuilder;

fn send_mail(mailer: &mut SmtpTransport, mail_adr: &str, mailbody: &String, code: &str) {
    let email = EmailBuilder::new()
        .to(mail_adr)
        .from("experiments@web3.foundation")
        .subject("Web3 Foundation Validator Selection Study")
        .text(mailbody.replace("{CODE}", code))
        .build()
        .unwrap();

    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }
}

fn main() {
    let username = match std::env::var("APP_USER") {
        Ok(val) => val.to_string(),
        Err(_) => panic!("Missing APP_USER"),
    };
    let password = match std::env::var("APP_PASS") {
        Ok(val) => val.to_string(),
        Err(_) => panic!("Missing APP_PASS"),
    };
    let mailbody: String = std::fs::read_to_string("mail.tmpl").unwrap().parse().unwrap();
    let mut mailer = SmtpTransport::new(
    SmtpClient::new_simple("smtp.gmail.com").unwrap()
    .credentials(Credentials::new(username, password)));

    let participant_code: String = std::fs::read_to_string("mail_code.csv").unwrap().parse().unwrap();
    let mut reader = csv::Reader::from_reader(participant_code.as_bytes());
    for record in reader.records() {
        let record = record.unwrap();
        let mailadr = &record[0];
        let code = &record[1];
        println!(
            "Mailing {} with code {}",
            &mailadr,
            &code
        );
        send_mail(&mut mailer,mailadr, &mailbody, code);
    };
}