#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use crate::eth_main::run_eth;




use crate::config_hand::{config_info, config};
use std::collections::{HashMap, BTreeMap};

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



////    获取配置文件中的信息
    let (conf,hk) = config_info();
//
//    println!("{:?}",conf);
//    println!("----------------");
//    println!("{:?}",hk);

//
////    运行实时分析
    run_eth(conf,hk);
//
//


//    send_email::send_email("邮件标题subject".to_string(),"邮件内容body".to_string(),conf.smtp_address, conf.smtp_from_username, conf.smtp_from_password, conf.smtp_to_usernames)






//    error_log::err_log("test error info123");



}
