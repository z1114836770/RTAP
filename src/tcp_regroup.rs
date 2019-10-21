extern crate chrono;

use std::collections::BTreeMap;
use crate::eth_2_struct::{eth_2_ip_tcp, tcp_regroup_struct, ipv4_tcp_data, tcp_head, tcp_session, tcp_node};
use std::borrow::BorrowMut;


use chrono::prelude::*;
use std::cmp::Ordering;
use pcap::Packet;


pub fn tcp_regroud(ip_map:&mut BTreeMap<String, tcp_session>, ip_tcp:eth_2_ip_tcp, packet:Packet) -> Option<eth_2_ip_tcp>{
    
//    客户端发送数据包key
    let client_send_key = format!("{}|{}|{}|{}",
                           ip_tcp.ip_head.source_address,
                            ip_tcp.tcp_head.src_port,
                            ip_tcp.ip_head.destination_address,
                            ip_tcp.tcp_head.dst_port
    );
    
//    服务器发送的数据包key
    let server_send_key  = format!("{}|{}|{}|{}",
                                   ip_tcp.ip_head.destination_address,
                                   ip_tcp.tcp_head.dst_port,
                                   ip_tcp.ip_head.source_address,
                                   ip_tcp.tcp_head.src_port
    );
    
    
//    客户端发送的数据包key 和 服务器端发送的数据包key 应该是相同的。
//    同一个数据包这两个值是不同的
    
    
//    如果一个数据包的这两种key都不存在则说明需要创建tcp会话
//    如果client_send_key存在则说明当前数据包为客户端发送的数据包
//    如果server_send_key存在则说明当前数据包为服务器发送的数据包
    if !ip_map.contains_key(&client_send_key) && !ip_map.contains_key(&server_send_key) {
        let tcp_session = tcp_session{
            src_ip: ip_tcp.ip_head.source_address,
            src_port: ip_tcp.tcp_head.src_port,
            des_ip: ip_tcp.ip_head.destination_address,
            des_port: ip_tcp.tcp_head.dst_port,
            create_time: 0,
            update_time: 0,
            data_len: 0,
            send_nods: Option::from(Box::new(tcp_node{
                syn: i32::from_str_radix( &ip_tcp.tcp_head.flags[4..=4] ,2).unwrap(),
                fin: i32::from_str_radix( &ip_tcp.tcp_head.flags[5..=5] ,2).unwrap(),
                seq: ip_tcp.tcp_head.sequence_number,
                len: packet.data.len() as i32,
                prev_tcp_node: None,
                netx_tcp_node: None,
                save: false,
                data:packet.data.to_vec(),
                data_lib: "".to_string()
            })),
            get_nods: None
        };
        ip_map.borrow_mut().insert(client_send_key,tcp_session);
    }else if ip_map.contains_key(&client_send_key) {
        let tcp_node =tcp_node{
            syn: i32::from_str_radix( &ip_tcp.tcp_head.flags[4..=4] ,2).unwrap(),
            fin: i32::from_str_radix( &ip_tcp.tcp_head.flags[5..=5] ,2).unwrap(),
            seq: ip_tcp.tcp_head.sequence_number,
            len: packet.data.len() as i32,
            prev_tcp_node: None,
            netx_tcp_node: None,
            save: false,
            data:packet.data.to_vec(),
            data_lib: "".to_string()
        };

        let tcp_session = ip_map.get_mut(&client_send_key).unwrap();
        let  send_nod = tcp_session.send_nods.as_ref().unwrap();
        let ar = send_nod.netx_tcp_node.as_ref();
        if(ar.is_none()){
            let mut netx_tcp_node = ar.unwrap();
            netx_tcp_node = &Box::new(tcp_node);
        }


    }else if ip_map.contains_key(&server_send_key) {
        
    }else{
        println!("不应该处理该情况，请联系管理员");
    }
    
    None
}
