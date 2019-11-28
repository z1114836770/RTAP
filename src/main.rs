#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use crate::eth_main::run_eth;

use std::thread;
use std::time::Duration;



use crate::config_hand::{config_info, config};
use std::collections::{HashMap, BTreeMap};
use std::borrow::{Borrow, BorrowMut};
use std::io::{Write, Read};
use uuid::Uuid;
use std::ops::Deref;
use crate::tools::time_stamp;
use crate::eth_2_struct::tcp_session;
use std::sync::{Arc, Mutex};

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
pub mod data_tmp;


fn main() {

//新开一个线程用于记录是否超时的情况
//每次接收到数据包就把更新时间发送到新的线程内
//在新线程内循环一直检查是否有超时情况
// 发现有超时情况就将超时的标识符发送有另一个线程，
// 另一个线程则将这个数据从会话集合中删除
 let mut tcp_map:Arc<Mutex<BTreeMap<String, tcp_session>>> = BTreeMap::new();


// let counter:Arc<Mutex<tcp_session>> = Arc::new(Mutex::new(tcp_session));

//新开线程 用于处理 超时的tcp重组数据

 let tcp_map_2 = tcp_map.clone();

 let handle = thread::spawn(move || {
  tcp_map_2.clone();
 });
 println!("111111111");
 handle.join();


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

// mongo_test();
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
