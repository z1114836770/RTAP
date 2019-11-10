#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use crate::eth_main::run_eth;




use crate::config_hand::{config_info, config};
use std::collections::{HashMap, BTreeMap};
use std::borrow::{Borrow, BorrowMut};

pub mod tools;
pub mod eth_2_struct;
pub mod eth_format;
pub mod eth_main;
pub mod eth_2_hand;
pub mod config_hand;
pub mod hk;
pub mod send_email;
pub mod error_log;
pub mod ip_regroup;
pub mod tcp_regroup;


fn main() {


//    let mut a = [71, 69, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58, 32, 119, 119, 119, 46, 98, 97, 105, 100, 117, 46, 99, 111, 109, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 99, 117, 114, 108, 47, 55, 46, 53, 50, 46, 49, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42, 13, 10, 13, 10];
//    a.iter_mut().map(|x| *x as u8);
//    println!("{:?}", String::from_utf8(a.to_vec()));

////    获取配置文件中的信息
// let (conf,hk) = config_info();
//

//
////    运行实时分析
//run_eth(conf,hk);
//
//

// 发送邮件
//    send_email::send_email("邮件标题subject".to_string(),"邮件内容body".to_string(),conf.smtp_address, conf.smtp_from_username, conf.smtp_from_password, conf.smtp_to_usernames)






//    error_log::err_log("test error info123");

 mongo_test();
}



fn mongo_test() {
 let client = Client::connect("localhost", 27017)
     .expect("Failed to initialize standalone client.");

 let coll = client.db("test").collection("movies");

 let doc = doc! {
        "title": "Jaws",
        "array": [ 1, 2, 3 ],
    };

 // Insert document into 'test.movies' collection
 coll.insert_one(doc.clone(), None)
     .ok().expect("Failed to insert document.");

 // Find the document and receive a cursor
 let mut cursor = coll.find(Some(doc.clone()), None)
     .ok().expect("Failed to execute find.");

 let item = cursor.next();

 // cursor.next() returns an Option<Result<Document>>
 match item {
  Some(Ok(doc)) => match doc.get("title") {
   Some(&Bson::String(ref title)) => println!("{}", title),
   _ => panic!("Expected title to be a string!"),
  },
  Some(Err(_)) => panic!("Failed to get next from server!"),
  None => panic!("Server returned no results!"),
 }
}
