#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use crate::eth_main::run_eth;




use crate::config_hand::config_info;

pub mod tools;
pub mod eth_2_struct;
pub mod eth_format;
pub mod eth_main;
pub mod eth_2_hand;
pub mod config_hand;
pub mod hk;
pub mod send_email;

fn main() {

////    获取配置文件中的信息
//    let (conf,hk) = config_info();
//
////    运行实时分析
//    run_eth(conf,hk);
//
//
//
//    env_logger::builder()
//        .filter_level(log::LevelFilter::Trace)
//        .init();
//
//    trace!("some trace log");
//    debug!("some debug log");
//    info!("some information log");
//    warn!("some warning log");
//    error!("some error log");

    send_email::send_email()

}
