use pcap::{Packet, Savefile};

use crate::eth_2_hand::hand_eth_2;
use crate::eth_format::eth_format;
use crate::eth_2_struct::{eth_2_struct, eth_2_ip_tcp, eth_2_head, ipv4_head, tcp_head, arp};
use crate::tools::{ u8_to_bit, u8_to_0x, u8s_to_0xs};
use crate::hk::hadn_hk;
use std::collections::{HashMap, BTreeMap};
use crate::config_hand::config;
use core::fmt::Debug;
use std::fmt::Formatter;
use crate::ip_regroup::ip_regroup;


//开始进入eth抓包
pub fn run_eth(conf:config,hk:HashMap<String,String>){

//    分片重组集合
    let mut groud:BTreeMap<String,Vec<eth_2_ip_tcp>> = BTreeMap::new();

//    开始抓包
    let mut cap = pcap::Capture::from_device(conf.net.as_str()).unwrap().open().unwrap();


//    let hk_map = hk_map();

    let mut savefile = cap.savefile("hk.pcap").unwrap();
    while let Ok(packet) = cap.next() {
//        println!("got packet! {:?}", packet);
//        savefile.write(&packet);

        analyze_ethernet(&packet,&mut savefile,&conf,&hk,&mut groud);

    }
}


//分析以太网数据包
fn analyze_ethernet(packet:&Packet, savefile:&mut Savefile, conf:&config,hk_map:&HashMap<String,String>, groud:&mut BTreeMap<String,Vec<eth_2_ip_tcp>>) {

    let data= packet.data;
    match eth_format::hand_eth_format(data) {
        eth_format::ETHERNET_II => {
//            println!("ETHERNET_II");
            let eth_2_info = hand_eth_2(data);
            match eth_2_info {
                eth_2_struct::ETH_IP_TCP(x) => {

//                    当只检查的源IP数量大于0的时候
//                    判断数据包的源IP是否位于只检查源IP中
//                    如果数据包中源IP不在只检查源IP中
//                    结束
                    if conf.check_source_ip.len() > 0 {
                        if !conf.check_source_ip.contains(&x.ip_head.source_address){
                            return;
                        }
//                     当不检查源IP数量大于0的时候
//                     判断数据包中源IP是否位于不检查源IP中
//                     如果数据包中源IP在不检查源IP中
//                        结束
                    }else if conf.not_check_source_ip.len() > 0 {
                        if conf.not_check_source_ip.contains(&x.ip_head.source_address){
                            return
                        }
                    }

//                    当只检查的源端口数量大于0的时候
//                    判断数据包中源端口是否位于只检查源端口中
//                    如果数据包中源端口不在只检查源端口中
//                    结束
                    if conf.check_source_port.len() > 0{
                        if !conf.check_source_port.contains(&format!("{}",&x.tcp_head.src_port)) {
                            return
                        }
//                        当不检查源端口数量大于0的时候
//                        判断数据包中源端口是否位于不检查源端口中
//                        如果数据包中源端口在不检查源端口中
//                        结束
                    }else if conf.not_check_source_port.len() > 0 {
                        if conf.not_check_source_port.contains(&format!("{}",&x.tcp_head.src_port)) {
                            return
                        }
                    }

//                    当只检查的目标IP数量大于0的时候
//                    判断数据包中目标IP是否位于只检查目标IP中
//                    如果数据包中目标IP不在只检查目标IP中
//                    结束
                    if conf.check_dst_ip.len() > 0 {
                        if !conf.check_dst_ip.contains(&x.ip_head.destination_address){
                            return
                        }
//                        当不检查的目标IP数量大于0的时候
//                        判断数据包中目标IP是否位于不检查目标IP中
//                        如果数据包中目标IP在不检查目标IP中
//                        结束
                    }else if conf.not_check_dst_ip.len() > 0 {
                        if conf.not_check_dst_ip.contains(&x.ip_head.destination_address){
                            return
                        }
                    }


//                    当只检查的目标端口数量大于0的时候
//                    判断数据包中目标端口是否位于只检查目标端口中
//                    如果数据包中目标端口不在只检查目标端口中
//                    结束
                    if conf.check_dst_port.len() > 0 {
                        if !conf.check_dst_port.contains(&format!("{}",&x.tcp_head.dst_port)) {
                            return
                        }
//                        当不检查的目标端口数量大于0的时候
//                        判断数据包中目标端口是否鱼尾不检查目标端口中
//                        如果数据包中目标端口在不检查目标端口中
//                        结束
                    }else if conf.not_check_dst_port.len() > 0 {
                        if conf.not_check_dst_port.contains(&format!("{}", &x.tcp_head.dst_port)){
                            return
                        }
                    }



                    println!("{:?}",x);

//                    tcp层数据重组

//                    ip层数据重组
                    if let Some(ip_tcp) = ip_regroup(groud,x) {
                        let info = format!("{:?}",ip_tcp);
//                            println!("{}",info);
//                        println!("{:?}",info);


//                            判断是否为恶意请求，将恶意数量包 保存到本地
                        if hadn_hk(&info,hk_map) {
                            savefile.write(&packet);
                        }

                    }

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

