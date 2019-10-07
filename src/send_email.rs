use lettre_email::{Email, Address, Mailbox};
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use std::path::Path;
use crate::error_log::err_log;

pub fn send_email(subject:String, body:String, smtp_address:String, smtp_from_username:String, smtp_from_password:String, smtp_to_usernames:Vec<String>) {
//    let email = Email::builder()
//        .to("1114836770@qq.com")
//        .from("zhangbeifeng@haoaninfo.com")
//        .subject("subject")
//        .html("<h1>Hi there</h1>")
//        .text("message")
////        .attachment_from_file(Path::new("myAttachement.pdf"), None, &mime::APPLICATION_PDF)
////        .unwrap()
//        .build()
//        .unwrap();
//
//
//
//
//    let creds = Credentials::new(
//        "zhangbeifeng@haoaninfo.com".to_string(),
//        "bei456!!".to_string(),
//    );
//
//    // Open connection to gmail
//    let mut mailer = SmtpClient::new_simple("smtp.mxhichina.com")
//        .unwrap()
//        .credentials(creds)
//        .transport();
//
//
//
//    // Send the email
//    let result = mailer.send(email.into());
//
//    if result.is_ok() {
//        println!("Email sent");
//    } else {
//        println!("Could not send email: {:?}", result);
//    }
//
//    assert!(result.is_ok());





    for smtp_to_username in smtp_to_usernames {
        match  Email::builder()
            .to(Mailbox::new(smtp_to_username.clone()))
            .from(Mailbox::new(smtp_from_username.clone()))
            .subject(subject.clone())
            .html(body.clone())
            .text("message")
            .build() {
            Ok(email) => {

                let creds = Credentials::new(
                    smtp_from_username.to_string(),
                    smtp_from_password.to_string(),
                );

                match SmtpClient::new_simple(smtp_address.clone().as_str()) {
                    Ok(smtp) => {
                        let mut mailer = smtp.credentials(creds)
                            .transport();
                        let result = mailer.send(email.into());
                        if result.is_ok() {
                            println!("Email sent");
                            err_log("Email sent");
                        } else {
                            println!("发送邮件失败");
                            err_log("发送邮件失败");
                        }
                    }
                    _ => {
                        println!("创建SMTP客户端失败");
                        err_log("创建SMTP客户端失败");
                    }
                }
            }
            _ => {
                println!("创建邮件失败");
                err_log("创建邮件失败");
            }
        }
    }

}