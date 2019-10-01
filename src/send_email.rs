extern crate lettre;
extern crate lettre_email;

#[macro_use]
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
#[macro_use]
use lettre::EmailTransport;

use lettre::SmtpTransport;
use lettre::smtp::ConnectionReuseParameters;

pub fn send_email() {
    let send_to = vec!["1114836770@qq.com"];
    let send_from = "zhangbeifeng@haoaninfo.com";
    let passwd = "bei456!!";
    let subject = "mail subject";
    let body = "mail body test";
    let domian = "smtp.mxhichina.com";//smtp.sina.com
    //=======

    let mut builder = EmailBuilder::new();
    for to in send_to {
        builder.add_to(to);
    }

    builder.add_from(send_from);
    builder.set_subject(subject);
    builder.set_body(body);
    let email = builder.build().unwrap();


    let mut mailer = SmtpTransport::simple_builder(domian).unwrap()
        .smtp_utf8(true)
        .hello_name(ClientId::Domain("localhost".to_string()))
        .credentials(Credentials::new(send_from.to_string(), passwd.to_string()))
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build();

    let result = mailer.send(&email);
    if !result.is_ok() {
        println!("send_email() get error: {:#?}", result);
    }
    mailer.close();
}