use pcap::{Packet, Savefile};

use crate::eth_2_hand::hand_eth_2;
use crate::eth_format::eth_format;
use crate::eth_2_struct::{eth_2_struct, eth_2_ip_tcp, eth_2_head, ipv4_head, tcp_head, arp, tcp_session};
use crate::tools::{ u8_to_bit, u8_to_0x, u8s_to_0xs};
use crate::hk::hadn_hk;
use std::collections::{HashMap, BTreeMap};
use crate::config_hand::config;
use core::fmt::Debug;
use std::fmt::Formatter;
use crate::ip_regroup::ip_regroup;
use crate::tcp_regroup::tcp_regroud;
use std::ops::Deref;
use crate::data_tmp::write_data_tmp;


//开始进入eth抓包
pub fn run_eth(conf:config,hk:HashMap<String,String>){

//    分片重组集合
    let mut groud:BTreeMap<String,Vec<eth_2_ip_tcp>> = BTreeMap::new();

//    开始抓包
    let mut cap = pcap::Capture::from_device(conf.net.as_str()).unwrap().open().unwrap();

//    tcp重组存放的map
    let mut tcp_map:BTreeMap<String, tcp_session> = BTreeMap::new();

//    let hk_map = hk_map();

    let mut savefile = cap.savefile("hk.pcap").unwrap();
    let mut num = 0;
    while let Ok(packet) = cap.next() {
//        println!("{}",num);
//        println!("got packet! {:?}", packet);
//        savefile.write(&packet);

        analyze_ethernet(&packet,&mut savefile,&conf,&hk,&mut tcp_map);
        println!("{}",tcp_map.len());
//        if tcp_map.len() == 20 {
//            for (k,v) in &tcp_map {
//                write_data_tmp(&k.replace(".","_"),format!("{:?}",v).as_bytes());
//
//            }
//
//        }
    }
}


//分析以太网数据包
fn analyze_ethernet(packet:&Packet, savefile:&mut Savefile, conf:&config,hk_map:&HashMap<String,String>, tcp_map:&mut BTreeMap<String,tcp_session>) {

    let data= packet.data;
    match eth_format::hand_eth_format(data) {
        eth_format::ETHERNET_II => {
//            println!("ETHERNET_II");
            let eth_2_info = hand_eth_2(data);
            match eth_2_info {
                eth_2_struct::ETH_IP_TCP(x) => {
//                    只要满足一个只检查的条件就可以
//                    未满足任何一个只检查条件，
//                    同时只检查条件中只要有一条长度不为0就结束
                    if !conf.check_source_ip.contains(&x.ip_head.source_address)
                        && !conf.check_source_port.contains(&format!("{}",&x.tcp_head.src_port))
                        && !conf.check_dst_ip.contains(&x.ip_head.destination_address)
                        && !conf.check_dst_port.contains(&format!("{}",&x.tcp_head.dst_port)){

                        if conf.check_source_ip.len() != 0
                            || conf.check_source_port.len() != 0
                            || conf.check_dst_ip.len() != 0
                            || conf.check_dst_port.len() != 0{
                            return
                        }
//                    只要满足一个不检查的条件就结束
                    }else if conf.not_check_source_ip.contains(&x.ip_head.source_address) ||
                        conf.not_check_source_port.contains(&format!("{}",&x.tcp_head.src_port)) ||
                        conf.not_check_dst_ip.contains(&x.ip_head.destination_address) ||
                        conf.not_check_dst_port.contains(&format!("{}",&x.tcp_head.dst_port)){
                        return
                    }




//                    println!("{:?}",x);




                    tcp_regroud(tcp_map, x);
//                    tcp_regroud(ip_map:&mut BTreeMap<String, tcp_session>, ip_tcp:eth_2_ip_tcp, data:&[u8]);




//                    tcp层数据重组

//                    ip层数据重组
//                    if let Some(ip_tcp) = ip_regroup(groud,x) {
//                        let info = format!("{:?}",ip_tcp);
////                            println!("{}",info);
////                        println!("{:?}",info);
//
//
////                            判断是否为恶意请求，将恶意数量包 保存到本地
//                        if hadn_hk(&info,hk_map) {
//                            savefile.write(&packet);
//                        }
//
//                    }

                }
                eth_2_struct::ETH_IP_UDP(udp) => {
//                    println!("{:?}",udp);
                    let info = format!("{:?}",udp);
                }
                _ =>{
                    println!("")
                }
            }

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

