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

fn main() {




    let (conf,hk) = config_info();

    run_eth(conf,hk);



    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

}
