extern crate chrono;

use std::collections::{BTreeMap, HashMap};
use crate::eth_2_struct::{eth_2_ip_tcp, tcp_regroup_struct, ipv4_tcp_data, tcp_head, tcp_session, tcp_data, ip_tcp_data};
use std::borrow::BorrowMut;


use chrono::prelude::*;
use std::cmp::Ordering;
use std::io::Write;
use std::ops::Deref;
use std::process::exit;
use uuid::Uuid;
use crate::data_tmp::write_data_tmp;
use crate::tools::time_stamp;


pub fn tcp_regroud(tcp_map:&mut BTreeMap<String, tcp_session>, ip_tcp:eth_2_ip_tcp) -> Option<eth_2_ip_tcp>{

//    数据长度超过制定长度则将数据进行存盘
    let data_limit_len = 2400;



//    flag 000010 第五位为 1 请求发起
//    flag 000100 第四位为 1 重置
//    flag 000001 第六位为 1 重组

//    要存储到第一个有效数据的序列号
//    要存储到最后一个有效数据的序列号

//    存储到有效数据包长度
//    重组是结束位的序列号减去请求发起时的序列号 等于这个长度

//    第三次握手的包的序列号为客户端seq
//    第三次握手的包的确认号为服务端seq


//    -------------

//    第一次接收到的数据段一定是客户端发起的
//    将客户端的ip+客户端端口+服务器ip+服务器端口
//    作为key存储起来

//    当后续的数据包到达
//    client_key和存储中一致 表示为客户端发送的数据包
//    sercer_key和存储中一致 表示为服务端发送的数据包

    let client_key = format!("{}|{}|{}|{}",
                         ip_tcp.ip_head.source_address,
                         ip_tcp.tcp_head.src_port,
        ip_tcp.ip_head.destination_address,
        ip_tcp.tcp_head.dst_port
    );

    let server_key = format!("{}|{}|{}|{}",
                         ip_tcp.ip_head.destination_address,
                         ip_tcp.tcp_head.dst_port,
                         ip_tcp.ip_head.source_address,
                         ip_tcp.tcp_head.src_port
    );



//    当client_key 和 server_key都不存 并且 数据包为 请求发起包
//    添加client_key到存储中
    if !tcp_map.contains_key(client_key.as_str()) && !tcp_map.contains_key(server_key.as_str())
        &&     &ip_tcp.tcp_head.flags[4..=4] == "1"
    {
//        进到这里的 一定为 请求发起的第一个数据包

//        存储client_key 并创建tcp_session
        tcp_map.insert(client_key,tcp_session{
            src_ip: ip_tcp.ip_head.source_address,
            src_port: ip_tcp.tcp_head.src_port,
            des_ip: ip_tcp.ip_head.destination_address,
            des_port: ip_tcp.tcp_head.dst_port,
            create_time: 0,
            update_time: 0,
            client_data_len: 0,
            client_syn_seq: ip_tcp.tcp_head.sequence_number,
            client_fin_seq: 0,
            client_fin: false,
            client_save: false,
            client_data_down: false,
            client_nods: Default::default(),
            server_data_len: 0,
            server_syn_seq: 0,
            server_fin_seq: 0,
            server_fin: false,
            server_save: false,
            server_data_down: false,
            server_nods: Default::default()
        });
    }
//        当client_key存在表示 客户端数据包
    else if tcp_map.contains_key(client_key.as_str()) {
//        进到这里的 二种情况
//        2、正式客户端发送数据包
//        3、客户端关闭数据包
//        4 客户端发送的复位包

//        3
        if &ip_tcp.tcp_head.flags[5..=5] == "1" {
            let tcp_session = tcp_map.get_mut(client_key.as_str()).unwrap();
            tcp_session.client_fin_seq = ip_tcp.tcp_head.sequence_number;
//            判断服务端结束序列号是否已有 则进入重组
            if tcp_session.server_fin_seq != 0 {
                println!("z{}",1);
                let res_ip_tcp_data = tcp_regroup_run(tcp_session);
                match res_ip_tcp_data {
                    Some(ip_tcp_data) => {
                        tcp_map.remove(client_key.as_str());
                        println!("remove1");
                    }
                    None => {
                        println!("没有完成重组")
                    }
                }
                println!("z{}",11);
            }
        }
//            4
        else if &ip_tcp.tcp_head.flags[3..=3] == "1" {
            tcp_map.remove(client_key.as_str());
            println!("remove2");
        }
//            2
        else {
            let tcp_session = tcp_map.get_mut(client_key.as_str()).unwrap();
            let client_nods = tcp_session.client_nods.borrow_mut();
            if !client_nods.contains_key(&ip_tcp.tcp_head.sequence_number) && ip_tcp.data_bit.len() > 0{
//                每次添加数据包更新时间
                tcp_session.update_time = i64::from_str_radix(&time_stamp(),10).unwrap();

//                将每次新数据包到来的数据长度相加
                tcp_session.client_data_len += ip_tcp.data_bit.len() as i64;
//                如果到达的数据长度大于制定长度,则将数据进行存盘
//                if tcp_session.client_data_len >= data_limit_len {
//                    let tmp_data_lib = format!("{:?}",Uuid::new_v4()).replace("-","");
//                    write_data_tmp(&tmp_data_lib, &ip_tcp.data_bit);
//                    client_nods.insert(ip_tcp.tcp_head.sequence_number,tcp_data{
//                        data_len: 0,
//                        data: vec![],
//                        data_lib: tmp_data_lib
//                    });
//                }else {
                    client_nods.insert(ip_tcp.tcp_head.sequence_number,tcp_data{
                        data_len: 0,
                        data: ip_tcp.data_bit.clone(),
                        data_lib: "".to_string()
                    });
//                }

                //            判断客户端和服务端的结束数据包是否都已经到达过
                if tcp_session.client_fin_seq !=0 && tcp_session.server_fin_seq != 0 {
                    println!("z{}",2);
                    let res_ip_tcp_data = tcp_regroup_run(tcp_session);
                    match res_ip_tcp_data {
                        Some(ip_tcp_data) => {
                            tcp_map.remove(client_key.as_str());
                            println!("remove3");
                        }
                        None => {
                            println!("没有完成重组")
                        }
                    }
                    println!("z{}",22);
                }
            }


        }

    }
//        当server_key存在表示 服务端数据包
    else if tcp_map.contains_key(server_key.as_str()) {
//        进到这里的 三种情况
//        1、服务端响应客户端发起的请求包
//        2、正式服务端发送数据包
//        3、服务端关闭数据包
//        4 服务端发送的复位包

//        1
        if &ip_tcp.tcp_head.flags[4..=4] == "1"{
            let tcp_session = tcp_map.get_mut(server_key.as_str()).unwrap();
            tcp_session.server_syn_seq = ip_tcp.tcp_head.sequence_number;
        }
//        3
        else if &ip_tcp.tcp_head.flags[5..=5] == "1" {
            let tcp_session =tcp_map.get_mut(server_key.as_str()).unwrap();
//            服务端结束序列号
            tcp_session.server_fin_seq = ip_tcp.tcp_head.sequence_number;
//            判断客户端结束序列号是否已有 则进入重组
            if tcp_session.client_fin_seq != 0 {
                println!("z{}",3);
                let res_ip_tcp_data = tcp_regroup_run(tcp_session);
                match res_ip_tcp_data {
                    Some(ip_tcp_data) => {
                        tcp_map.remove(server_key.as_str());
                        println!("remove4");
                    }
                    None => {
                        println!("没有完成重组")
                    }
                }
                println!("z{}",33);
            }
        }
//            4
        else if &ip_tcp.tcp_head.flags[3..=3] == "1" {
            tcp_map.remove(server_key.as_str());
            println!("remove5");
        }
//            2
        else {
            let tcp_session = tcp_map.get_mut(server_key.as_str()).unwrap();
            let server_nods = tcp_session.server_nods.borrow_mut();
            if !server_nods.contains_key(&ip_tcp.tcp_head.sequence_number) && ip_tcp.data_bit.len() > 0{
//                将每次的数据长度相加
                tcp_session.server_data_len += ip_tcp.data_bit.len() as i64;
                server_nods.insert(ip_tcp.tcp_head.sequence_number, tcp_data{
                    data_len: 0,
                    data: ip_tcp.data_bit,
                    data_lib: "".to_string()
                });
                //            判断客户端和服务端的结束数据包是否都已经到达过
                if tcp_session.client_fin_seq !=0 && tcp_session.server_fin_seq != 0 {
                    println!("z{}",4);
                    let res_ip_tcp_data = tcp_regroup_run(tcp_session);
                    match res_ip_tcp_data {
                        Some(ip_tcp_data) => {
                            tcp_map.remove(server_key.as_str());
                            println!("remove6");
                        }
                        None => {
                            println!("没有完成重组")
                        }
                    }
                    println!("z{}",44);
                }
            }

        }

    }
//        表示为重发包 或则 开启之前的数据包
//        客户忽略
//        不重要
    else {

    }

    None
}



//进行数据包重组
fn tcp_regroup_run(tcp_session:&mut tcp_session) -> Option<ip_tcp_data>{
    let client_seq_len = &tcp_session.client_fin_seq - &tcp_session.client_syn_seq;
    let server_seq_len = &tcp_session.server_fin_seq - &tcp_session.server_syn_seq;
    let client_nodes = &tcp_session.client_nods;
    let mut client_keys = client_nodes.clone().keys();
    let server_nodes = &tcp_session.server_nods;
    let server_keys = server_nodes.clone().keys();

    println!("{:?}",client_keys);
//    将客户端的key转换成vec
    let mut new_client_keys = vec![];
    client_keys.for_each(|key|
        new_client_keys.push(key )
    );
    println!("{:?}",new_client_keys);
//    将新key进行升序
    new_client_keys.sort();
//    客户端重组后的数据vec
    let mut client_data_bit = vec![];
    for key in new_client_keys {
//        key 是序列号 需要减去 syn_seq 再减去 发起请求的 1 则为 数据开始的下标
        let index = (key - &tcp_session.client_syn_seq - 1) as usize;
//        如果 数据开始的下标正好是长度 那么直接在后面进行追加
        if client_data_bit.len() == index {
            client_data_bit.extend(&client_nodes.get(key).unwrap().data);
        }
//            如果下标的位置 小于 已存在的数据的长度 则需要在已存在的数据中 删除 下标后面的数据 再进行追加
        else if client_data_bit.len() > index {
            client_data_bit.truncate(index);
            client_data_bit.extend(&client_nodes.get(key).unwrap().data);
        }
//            如果下标的位置 超过 已存在数据的长度 则表示数据还没有 到齐 则不进行重组
        else if  client_data_bit.len() < index {
            return None
        }
    }

    println!("{:?}",server_keys);
//    将服务端的ket转换成vec
    let mut new_server_keys = vec![];
    server_keys.for_each(|key|
          new_server_keys.push(key)
    );
    println!("{:?}",new_server_keys);
//    将新的服务端key进行排序
    new_server_keys.sort();
//    服务端重组后的数据vec
    let mut server_data_bit = vec![];
    for key in new_server_keys {
//        key 是序列号 需要减去 syn_seq 再减去 发起请求的 1 则为 数据开始的下标
        let index = (key - &tcp_session.server_syn_seq - 1) as usize;
//        如果 数据开始的下标正好是长度 那么直接在后面进行追加
        if server_data_bit.len() == index {
            server_data_bit.extend(&server_nodes.get(key).unwrap().data);
        }
//            如果下标的位置 小于 已存在的数据的长度 则需要在已存在的数据中 删除 下标后面的数据 再进行追加
        else if server_data_bit.len() > index {
            server_data_bit.truncate(index);
            server_data_bit.extend(&server_nodes.get(key).unwrap().data);
        }
//            如果下标的位置 超过 已存在数据的长度 则表示数据还没有 到齐 则不进行重组
        else if  server_data_bit.len() < index {
            return None
        }
    }

    let s_ip = &tcp_session.src_ip;
    let s_port = &tcp_session.src_port;
    let d_ip = &tcp_session.des_ip;
    let d_port = &tcp_session.des_port;
//    最后完成重组后的数据
    let ip_tcp_data = ip_tcp_data{
        s_ip: s_ip.clone(),
        s_port: *s_port,
        d_ip: d_ip.clone(),
        d_port: *d_port,
        client_data_bit: client_data_bit,
        client_data_lib: "".to_string(),
        server_data_bit: server_data_bit,
        server_data_lib: "".to_string()
    };
    println!("{:?}",ip_tcp_data);
    println!("{:?}",String::from_utf8(ip_tcp_data.clone().client_data_bit));
    println!("{:?}",String::from_utf8(ip_tcp_data.clone().server_data_bit));
    return Some(ip_tcp_data)
}