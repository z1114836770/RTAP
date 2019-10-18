extern crate chrono;

use std::collections::BTreeMap;
use crate::eth_2_struct::{eth_2_ip_tcp, tcp_regroup_struct, ipv4_tcp_data, tcp_head};
use std::borrow::BorrowMut;


use chrono::prelude::*;
use std::cmp::Ordering;


pub fn tcp_regroud(ip_map:&mut BTreeMap<String, tcp_regroup_struct>, ip_tcp:eth_2_ip_tcp, data:Vec<u8>) -> Option<eth_2_ip_tcp>{

//  发送包的key
    let source_ip_port_dest_ip_port = format!("{}/{}/{}/{}",
                                              &ip_tcp.ip_head.source_address,
                                              &ip_tcp.tcp_head.src_port,
                                              &ip_tcp.ip_head.destination_address,
                                              &ip_tcp.tcp_head.dst_port);
//    接收包的key
    let dest_ip_port_source_ip_port = format!("{}/{}/{}/{}",
                                              &ip_tcp.ip_head.source_address,
                                              &ip_tcp.tcp_head.src_port,
                                              &ip_tcp.ip_head.destination_address,
                                              &ip_tcp.tcp_head.dst_port);


//    判断两种形式的key在总集合ip_map中是否已经存在
//    如果不存在则按第一种方式的key创建ip_map对应的tcp_regroup_struct信息

//    如果存在则进行添加，
//    判断是否为最后一个包
//      如果是最后一个包到 则进行数据重组运算，
//      如果最后一个包到，但数据整体未完整，则以后到的每个数据包 都将进行数据重组运算


//    -------------------------

//    判断tcp重组类型是否已经创建
//    如果未创建则新建
    if !ip_map.contains_key(&source_ip_port_dest_ip_port) &&
        !ip_map.contains_key(&dest_ip_port_source_ip_port){
        let sequence_number = ip_tcp.tcp_head.sequence_number.clone();
        let data_len = data.len();
        

        let local_now_timestamp = Local::now().timestamp_millis();

        let tcp_regroup_struct = tcp_regroup_struct{
            all_tcp_data_len: data_len as i32,
            create_tcp_regroup_time: local_now_timestamp,
            new_in_tcp_time: local_now_timestamp,
            end_lable: 0,
            syn_seq: ip_tcp.tcp_head,
            fin_seq: tcp_head{
                src_port: 0,
                dst_port: 0,
                sequence_number,
                ack_number: 0,
                tcp_header_len: 0,
                flags: "".to_string(),
                window_size: 0,
                checksum: "".to_string(),
                urgent_pointer: 0,
                options: vec![]
            },
            send_tcps: vec![],
            res_tcps: vec![]
        };
        ip_map.borrow_mut() .insert(source_ip_port_dest_ip_port,tcp_regroup_struct);

//        判断tcp数据包是否为一个方向上面的数据
//        并判断是否意见存在，不存在则添加
    }else if !ip_map.contains_key(&source_ip_port_dest_ip_port) {

        let ipv4_tcp_data = ipv4_tcp_data{
            ip_head: ip_tcp.ip_head,
            tcp_head: ip_tcp.tcp_head,
            tcp_data: data
        };



        let s1_value = ip_map.get_mut(&source_ip_port_dest_ip_port).expect("读取ip_map数据异常");

        s1_value.all_tcp_data_len += data.len();

        if !s1_value.send_tcps.contains(&ipv4_tcp_data) {
            s1_value.new_in_tcp_time = Local::now().timestamp_millis();
            s1_value.send_tcps.push(ipv4_tcp_data);
        }

        let fin = ip_tcp.tcp_head.flags[5];
        if fin == 1 {
//          进行重组运算
            s1_value.fin_seq = ip_tcp.tcp_head;
            s1_value.end_lable = 1


        }
    }else if !ip_map.contains_key(&dest_ip_port_source_ip_port) {

        let ipv4_tcp_data = ipv4_tcp_data{
            ip_head: ip_tcp.ip_head,
            tcp_head: ip_tcp.tcp_head,
            tcp_data: data
        };

        let s1_value = ip_map.get_mut(&dest_ip_port_source_ip_port).expect("读取ip_map数据异常");

        s1_value.all_tcp_data_len += data.len();

        if !s1_value.res_tcps.contains(&ipv4_tcp_data) {
            s1_value.new_in_tcp_time = Local::now().timestamp_millis();
            s1_value.res_tcps.push(ipv4_tcp_data);
        }

        let fin = ip_tcp.tcp_head.flags[5];
        if fin == 1 {
//          进行重组运算
            s1_value.fin_seq = ip_tcp.tcp_head;
            s1_value.end_lable = 1

        }
    }

    None
}


//重组运算
fn tcp_regroud_ing(tcp_regroup_struct: &tcp_regroup_struct) -> Option<ipv4_tcp_data>{
    let all_tcp_data_len = &tcp_regroup_struct.all_tcp_data_len;
    let syn_seq = &tcp_regroup_struct.syn_seq;
    let fin_seq = &tcp_regroup_struct.fin_seq;
    let new_in_tcp_time = &tcp_regroup_struct.new_in_tcp_time;
    let create_tcp_regroup_time = &tcp_regroup_struct.create_tcp_regroup_time;
    let mut send_tcps = &tcp_regroup_struct.send_tcps;
    let mut res_tcps = &tcp_regroup_struct.res_tcps;


    let len = (fin_seq.sequence_number - syn_seq.sequence_number - ( all_tcp_data_len as i64)) as i32;
    let expend = new_in_tcp_time - create_tcp_regroup_time;


//  已到数据长度 和 应到数据长度不符合  直接结束
    if &len != all_tcp_data_len {
        return None;
    }




    send_tcps.sort_by_key(|x| x.tcp_head.sequence_number);
    res_tcps.sort_by_key(|x| x.tcp_head.sequence_number);

    let mut send_data = vec![];
    let mut res_data = vec![];

    for send_tcp in send_tcps {
        send_data.push(send_tcp.tcp_data);
    }

    for res_tcp in res_tcps {
        res_data.push(res_tcp.tcp_data)
    }


    None
}