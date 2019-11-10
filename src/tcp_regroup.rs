extern crate chrono;

use std::collections::{BTreeMap, HashMap};
use crate::eth_2_struct::{eth_2_ip_tcp, tcp_regroup_struct, ipv4_tcp_data, tcp_head, tcp_session, tcp_data};
use std::borrow::BorrowMut;


use chrono::prelude::*;
use std::cmp::Ordering;
use std::io::Write;
use std::ops::Deref;
use std::process::exit;


pub fn tcp_regroud(tcp_map:&mut BTreeMap<String, tcp_session>, ip_tcp:eth_2_ip_tcp) -> Option<eth_2_ip_tcp>{


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

//  数据
    let data = ip_tcp.data_bit.clone();
    let now_data_len = data.len() as i64;


//    当前使用的key
    let mut now_key = Option::None;
    
//    客户端发送的数据包key 和 服务器端发送的数据包key 应该是相同的。
//    同一个数据包这两个值是不同的


//    如果一个数据包的这两种key都不存在则说明需要创建tcp会话
//    如果client_send_key存在则说明当前数据包为客户端发送的数据包
//    如果server_send_key存在则说明当前数据包为服务器发送的数据包
    if !tcp_map.contains_key(&client_send_key) && !tcp_map.contains_key(&server_send_key) &&
//        查看是否未第一个包，防止完成重组后 延迟到达的包 占用内存
        i32::from_str_radix( &ip_tcp.tcp_head.flags[4..=4] ,2).unwrap() == 1 {
//        println!("create");
        let mut send_hash_map:HashMap<i64,tcp_data> = HashMap::new();
        let tcp_data = tcp_data{
            data_len: data.len() as i32,
            data:data.to_vec(),
            data_lib: "".to_string()
        };
//        第一个客户端数据包没有实际数据所有不存
//        send_hash_map.insert(ip_tcp.tcp_head.sequence_number, tcp_data);
        let mut server_hash_map:HashMap<i64,tcp_data> = HashMap::new();
        let tcp_session = tcp_session{
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
            client_nods: send_hash_map,

            server_data_len: 0,
            server_syn_seq: 0,
            server_fin_seq: 0,
            server_fin: false,
            server_save: false,
            server_data_down: false,
            server_nods: server_hash_map
        };
        &mut tcp_map.insert(client_send_key,tcp_session);
//        println!("{:?}",tcp_map);
//        now_key = Some(server_send_key);

//        判断数据包客户端发送的数据包
    }else if tcp_map.contains_key(&client_send_key) {
//        println!("client");
//                println!("{:?}",tcp_map);

//        获取tcp会话
        let tcp_session = tcp_map.get_mut(&client_send_key).unwrap();
//        判断客户端的数据包是否已经全部到达
//        如果未完成则接收客户端数据包并进行处理
        if tcp_session.client_data_down {
            now_key = Some(client_send_key);
        }else{

//        利用当前数据包序列号 判断是否已经存在
//        如果已经存在判断长度是否一致
//        如果新的数据包长度比原先的长 则删除原来的数据包存放新的数据包，同时更新客户端数据包总长度

//            tcp数据包中没有实际数据包长度的不存
            if data.len() > 0 {
                if tcp_session.client_nods.contains_key(&ip_tcp.tcp_head.sequence_number) &&
                    tcp_session.client_nods.get(&ip_tcp.tcp_head.sequence_number).unwrap().data_len < ( data.len() as i32){
//                tcp_session.client_data_len -= tcp_session.client_nods.get(&ip_tcp.tcp_head.sequence_number).unwrap().data_len;
                    tcp_session.client_nods.remove(&ip_tcp.tcp_head.sequence_number);
//                tcp_session.client_data_len += data.len();
                    tcp_session.client_nods.insert(ip_tcp.tcp_head.sequence_number, tcp_data{
                        data_len: data.len() as i32,
                        data:data,
                        data_lib: "".to_string()
                    });
//                    println!("save client insert");
                }else{
                    tcp_session.client_nods.insert(ip_tcp.tcp_head.sequence_number, tcp_data{
                        data_len: data.len() as i32,
                        data:data,
                        data_lib: "".to_string()
                    });
//                    println!("no save client insert");
                }
            }


//        判断是否为结束包
//        如过为结束包则更改标识信息
//            if i32::from_str_radix( &ip_tcp.tcp_head.flags[5..=5] ,2).unwrap()  == 1{
            if i32::from_str_radix( &ip_tcp.tcp_head.flags[2..=2] ,2).unwrap()  == 1{
                tcp_session.client_fin_seq = ip_tcp.tcp_head.sequence_number;
                tcp_session.client_fin = true;
                tcp_session.client_data_len = ip_tcp.tcp_head.sequence_number - tcp_session.client_syn_seq + now_data_len;
                now_key = Some(client_send_key);
            }

//            如果最后一个数据包已经到达则进行重组检查
//            if tcp_session.client_fin == true {
////                -1 是因为发起请求的客户端的第一个数据包没有数据 且 被服务端确认并 +1
//                let data_mast_len = tcp_session.client_fin_seq - tcp_session.client_syn_seq - 1;
//                let mut client_data:Vec<i16> = vec![];
//                for v in 0..data_mast_len{
//                    client_data.push(-1);
//                }
//                for (k,v) in &tcp_session.client_nods {
//                    //                -1 是因为发起请求的客户端的第一个数据包没有数据 且 被服务端确认并 +1
//                    let mut i = *k - tcp_session.client_syn_seq - 1;
//                    for vv in &v.data {
//                        client_data[(i as usize)] = *vv as i16;
//                        i = i + 1;
//                    }
//                }
//                if !client_data.contains( &-1){
//                    tcp_session.client_data_down = true;
//                    println!("f{:?}",client_data);
//                    let mut d:Vec<u8> = vec![];
//                    for client_datum in client_data {
//                        d.push(client_datum as u8);
//                    }
//
//                    println!("{:?}",String::from_utf8(d));
//                }
//
//            }
        }




    }else if tcp_map.contains_key(&server_send_key) {
//        println!("server");
        //                println!("{:?}",tcp_map);

//        获取tcp会话
        let tcp_session = tcp_map.get_mut(&server_send_key).unwrap();
//        判断客户端的数据包是否已经全部到达
//        如果未完成则接收客户端数据包并进行处理
        if tcp_session.server_data_down {
            now_key = Some(server_send_key);
        }else{

//            if tcp_session.server_nods.len() == 0 {
//            if tcp_session.client_syn_seq + 1== ip_tcp.tcp_head.ack_number {
            if i32::from_str_radix( &ip_tcp.tcp_head.flags[4..=4] ,2).unwrap() == 1  {
                println!("有");
                tcp_session.server_syn_seq = ip_tcp.tcp_head.sequence_number
            }


//        利用当前数据包序列号 判断是否已经存在
//        如果已经存在判断长度是否一致
//        如果新的数据包长度比原先的长 则删除原来的数据包存放新的数据包，同时更新客户端数据包总长度

//            tcp数据包中没有实际数据包长度的不存
            if data.len() > 0 {
                if tcp_session.server_nods.contains_key(&ip_tcp.tcp_head.sequence_number) &&
                    tcp_session.server_nods.get(&ip_tcp.tcp_head.sequence_number).unwrap().data_len < ( data.len() as i32){
//                tcp_session.client_data_len -= tcp_session.client_nods.get(&ip_tcp.tcp_head.sequence_number).unwrap().data_len;
                    tcp_session.server_nods.remove(&ip_tcp.tcp_head.sequence_number);
//                tcp_session.client_data_len += data.len();
                    tcp_session.server_nods.insert(ip_tcp.tcp_head.sequence_number, tcp_data{
                        data_len: data.len() as i32,
                        data:data,
                        data_lib: "".to_string()
                    });
//                    println!("save server insert");
                }else if !tcp_session.server_nods.contains_key(&ip_tcp.tcp_head.sequence_number){
                    tcp_session.server_nods.insert(ip_tcp.tcp_head.sequence_number, tcp_data{
                        data_len: data.len() as i32,
                        data:data,
                        data_lib: "".to_string()
                    });
//                    println!("no save server insert");
                }
            }


//        判断是否为结束包
//        如过为结束包则更改标识信息
//            if i32::from_str_radix( &ip_tcp.tcp_head.flags[5..=5] ,2).unwrap()  == 1{
            if i32::from_str_radix( &ip_tcp.tcp_head.flags[2..=2] ,2).unwrap()  == 1{
                tcp_session.server_fin_seq = ip_tcp.tcp_head.sequence_number;
                tcp_session.server_fin = true;
                tcp_session.server_data_len = ip_tcp.tcp_head.sequence_number - tcp_session.server_syn_seq + now_data_len;
                now_key = Some(server_send_key);
            }

//            如果最后一个数据包已经到达则进行重组检查
//            if tcp_session.server_fin == true {
////                -1 是因为发起请求的客户端的第一个数据包没有数据 且 被服务端确认并 +1
//                let data_mast_len = tcp_session.server_fin_seq - tcp_session.server_syn_seq ;
//                println!("z{}",data_mast_len);
//                let mut client_data:Vec<i16> = vec![];
//                for v in 0..data_mast_len{
//                    client_data.push(-1);
//                }
////                println!("b{:?}",client_data);
//                println!("{:?}",tcp_session);
//                for (k,v) in &tcp_session.server_nods {
//
//                    let mut i = *k - tcp_session.server_syn_seq ;
//                    println!("{}",i);
//                    for vv in &v.data {
//                        client_data[(i as usize)] = *vv as i16;
////                        client_data.
////                        client_data.insert(i as usize,*vv as i16);
//                        i = i + 1;
//                    }
//
//                }
//                if !client_data.contains( &-1){
//                    tcp_session.server_data_down = true;
//                    println!("f{:?}",client_data);
//                    let mut d:Vec<u8> = vec![];
//                    for client_datum in client_data {
//                        d.push(client_datum as u8);
//                    }
//
//                    println!("{:?}",String::from_utf8(d));
//                }
//            }
        }

    }else{
        println!("不应该处理该情况，请联系管理员");
        println!("有可能为已经完成重组包的 延时重发包");
        println!("有可能为启动之前已经建立起来的短连接收发包");
        println!("有可能为启动之前已经建立起来的长连接收发包");
//        println!("{:?}",ip_tcp);
//        println!("{}",client_send_key);
//        println!("{}",server_send_key);
    }


//    判断tcp数据包两个方向上是否都是已经结束
    if let Some(key) = now_key {
        let tcp_session = tcp_map.get(&key).unwrap();

//            如果最后一个数据包已经到达则进行重组检查
        if tcp_session.client_fin  && tcp_session.server_fin{
            println!("client regroud");
//                -1 是因为发起请求的客户端的第一个数据包没有数据 且 被服务端确认并 +1
//            let data_mast_len = tcp_session.client_fin_seq - tcp_session.client_syn_seq -1;
            let data_mast_len = tcp_session.client_data_len-1;
            let mut client_data:Vec<i16> = vec![];
            for v in 0..data_mast_len{
                client_data.push(-1);
            }
            for (k,v) in &tcp_session.client_nods {
                let mut i = *k - tcp_session.client_syn_seq - 1;
                for vv in &v.data {
                    client_data[(i as usize)] = *vv as i16;
                    i = i + 1;
                }
            }
            if !client_data.contains( &-1){
//                println!("f{:?}",client_data);
                let mut d:Vec<u8> = vec![];
                for client_datum in client_data {
                    d.push(client_datum as u8);
                }
                println!("{:?}",String::from_utf8(d));
            }
//        }
//
//
//
//
            println!("server regroud");
//        if tcp_session.server_fin == true {
//                -1 是因为发起请求的客户端的第一个数据包没有数据 且 被服务端确认并 +1


//            let data_mast_len = tcp_session.server_fin_seq - tcp_session.server_syn_seq ;
            println!("{}",tcp_session.server_fin_seq - tcp_session.server_syn_seq );
            println!("{:?}",tcp_session);
            let data_mast_len = tcp_session.server_data_len ;
            println!("z{}",data_mast_len);
            let mut client_data:Vec<i16> = vec![];
            for v in 0..data_mast_len{
                client_data.push(-1);
            }
            println!("{:?}",client_data);
            for (k,v) in &tcp_session.server_nods {

                if k < &tcp_session.server_syn_seq{

                    println!("server_syn_seq:{}",&tcp_session.server_syn_seq);
                    println!("k:{:?}",k);
                    exit(0);
                }
                let mut i = k - tcp_session.server_syn_seq ;
                for vv in &v.data {
                    client_data[(i as usize)] = *vv as i16;
                    i = i + 1;
                }
            }





            if !client_data.contains( &-1){
//                println!("f{:?}",client_data);
                let mut d:Vec<u8> = vec![];
                for client_datum in client_data {
                    d.push(client_datum as u8);
                }

                println!("{:?}",String::from_utf8(d));
            }



            tcp_map.remove(&key);
        }






    }
    
    
    
    None
}
