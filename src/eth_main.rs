use pcap::{Packet, Savefile};

use crate::eth_2_hand::hand_eth_2;
use crate::eth_format::eth_format;
use crate::eth_2_struct::{eth_2_struct, eth_2_ip_tcp, eth_2_head, ipv4_head, tcp_head, arp};
use crate::tools::{ u8_to_bit, u8_to_0x, u8s_to_0xs};
use crate::hk::hadn_hk;
use std::collections::HashMap;
use crate::config_hand::config;
use core::fmt::Debug;
use std::fmt::Formatter;


//开始进入eth抓包
pub fn run_eth(conf:config,hk:HashMap<String,String>){

//    开始抓包
    let mut cap = pcap::Capture::from_device(conf.net.as_str()).unwrap().open().unwrap();


//    let hk_map = hk_map();

//    let mut savefile = cap.savefile("test.pcap").unwrap();
    while let Ok(packet) = cap.next() {
//        println!("got packet! {:?}", packet);
//        savefile.write(&packet);

        analyze_ethernet(&packet,&conf,&hk);

    }
}


//分析以太网数据包
fn analyze_ethernet(packet:&Packet,conf:&config,hk_map:&HashMap<String,String>) {

    let data= packet.data;
    match eth_format::hand_eth_format(data) {
        eth_format::ETHERNET_II => {
//            println!("ETHERNET_II");
            let eth_2_info = hand_eth_2(data);
            match eth_2_info {
                eth_2_struct::ETH_IP_TCP(x) => {
                    println!("{:?}",x);
                    let info = format!("{:?}",x);
                    hadn_hk(&info,hk_map);
                }
                eth_2_struct::ETH_IP_UDP(udp) => {
//                    println!("{:?}",udp);
                    let info = format!("{:?}",udp);
                }
                _ =>{
                    println!("")
                }
            }

//            let info = format!("{:?}",eth_2_info);
//            println!("{:?}",info);
//            hadn_hk(&info,hk_map);
        }
        eth_format::NOVELL_ETHERNET => {
            println!("NOVELL_ETHERNET is not develop")
        }
        eth_format::IEEE_802_3_SAP => {
            println!("IEEE_802_3_SAP is not develop");
//            &savefile.write(packet);
        }
        eth_format::IEEE_802_3_SNAP => {
            println!("IEEE_802_3_SNAP is not develop")
        }
        eth_format::UNKNOWN =>{
            println!("UNKNOWN is not develop")
        }
    }
}

