use crate::eth_format::eth_format;
use crate::eth_2_struct::{eth_2_struct, eth_2_ip_tcp, eth_2_head, ipv4_head, tcp_head, arp, udp_head, eth_2_ip_udp};
use crate::tools::{ u8_to_bit, u8_to_0x, u8s_to_0xs};

// ETHERNET_II 以太网帧类型
pub fn hand_eth_2(data:&[u8]) -> eth_2_struct{
//    获取以太网帧内协议类型
    let protocol =u8s_to_0xs(&data[12..=13]).join("");
    match protocol.as_str() {
//        ipv4协议
        "0800" => {
//            ipv4上层协议
            let protocol = &data[14..][9];
            match protocol {
//                IGMP
                2 => {
                    eth_2_struct::UNKOWN("IGMP协议未开发".to_string())
                }
//                tcp协议
                6 => {
                    let eth_2_head = hand_eth_head(data);
                    let ipv4_head = hand_ipv4_head(&data[14..]);
                    let tcp_head = hand_tcp_head(&data[(14+ipv4_head.ip_pg_head_len) as usize..]);
                    let data_bit = data[((14 + ipv4_head.ip_pg_head_len + tcp_head.tcp_header_len) as usize)..(data.len() as usize)].to_vec();
                    let mut data_info = String::new();

                    match String::from_utf8(data_bit.clone()) {
                        Ok(s) => {
                            data_info = s
                        }
                        Err(e) => {
                            data_info = "".to_string()
                        }
                    }
                    eth_2_struct::ETH_IP_TCP(eth_2_ip_tcp{
                        eth_head: eth_2_head,
                        ip_head: ipv4_head,
                        tcp_head: tcp_head,
                        data_bit: data_bit,
                        data_info: data_info,
                    })
                }
//                udp协议
                17 => {
                    let eth_2_head = hand_eth_head(data);
                    let ipv4_head = hand_ipv4_head(&data[14..]);
                    let udp_head = hand_udp_head(&data[(14+ipv4_head.ip_pg_head_len) as usize..]);
                    let data_bit = data[((14 + ipv4_head.ip_pg_head_len + 8) as usize)..(data.len() as usize)].to_vec();
                    let mut data_info = String::new();

                    match String::from_utf8(data_bit.clone()) {
                        Ok(s) => {
                            data_info = s
                        }
                        Err(e) => {
                            data_info = "".to_string()
                        }
                    }

                    eth_2_struct::ETH_IP_UDP(eth_2_ip_udp{
                        eth_head: eth_2_head,
                        ip_head: ipv4_head,
                        udp_head: udp_head,
                        data_bit: data_bit,
                        data_info: data_info,
                    })
                }
                _ => {
                    eth_2_struct::UNKOWN(format!("IP上层协议未开发： {}",protocol))
                }
            }
        }
//      arp
        "0806" => {
            let arp = hand_arp(&data[14..]);
            eth_2_struct::ARP(arp)
        }
        _ => {
            eth_2_struct::UNKOWN(format!("eth 2 上层协议未开发  : {}",protocol))
        }
    }
}

//解析以太网头部
pub fn hand_eth_head(data:&[u8]) -> eth_2_head{
    eth_2_head{
        dst_mac:u8s_to_0xs(&data[0..=5]).join(":"),
        src_mac:u8s_to_0xs(&data[6..=11]).join(":"),
        data_type:u8s_to_0xs(&data[12..=13]).join("")
    }
}

//解析ipv4头部
pub fn hand_ipv4_head(data:&[u8]) -> ipv4_head{
    let ip_version__ip_head_len = u8_to_bit(&data[0]);
    let mut flags_flags_offset = u8_to_bit(&data[6]);
    flags_flags_offset.push_str(u8_to_bit(&data[7]).as_str());
    let ip_pg_head_len = i32::from_str_radix( &ip_version__ip_head_len[4..=7] ,2).unwrap() * 4;
    ipv4_head{
        ip_version: i32::from_str_radix( &ip_version__ip_head_len[0..=3] ,2).unwrap(),
        ip_pg_head_len: ip_pg_head_len,
        service_type: u8_to_bit(&data[1]),
        ip_pg_len: i32::from_str_radix(u8s_to_0xs(&data[2..=3]).join("").as_str(),16).unwrap(),
        identifier: u8s_to_0xs(&data[4..=5]).join(""),
        flags: flags_flags_offset[0..=2].to_string(),
        flags_offset: flags_flags_offset[3..=15].to_string(),
        time_to_live: data[8] as i32,
        protocol: data[9] as i32,
        header_checksum: u8s_to_0xs(&data[10..=11]).join(""),
        source_address: data[12..=15].iter().map(|x| format!("{}",x))
            .collect::<Vec<String>>().join("."),
        destination_address: data[16..=19].iter().map(|x| format!("{}",x))
            .collect::<Vec<String>>().join("."),
        options: data[20..(ip_pg_head_len as usize)].to_vec()
    }
}

//解析tcp头部
pub fn hand_tcp_head(data:&[u8]) -> tcp_head{
    let mut tcp_header_len__keep_flags =  u8_to_bit(&data[12]);
    tcp_header_len__keep_flags.push_str(u8_to_bit(&data[13]).as_str());
    let tcp_header_len = i32::from_str_radix( &tcp_header_len__keep_flags[0..=3] ,2).unwrap() * 4;

    tcp_head{
        src_port: i32::from_str_radix(u8s_to_0xs(&data[0..=1]).join("").as_str(),16).unwrap(),
        dst_port: i32::from_str_radix(u8s_to_0xs(&data[2..=3]).join("").as_str(),16).unwrap(),
        sequence_number: i64::from_str_radix(u8s_to_0xs(&data[4..=7]).join("").as_str(),16).unwrap(),
        ack_number: i64::from_str_radix(u8s_to_0xs(&data[8..=11]).join("").as_str(),16).unwrap(),
        tcp_header_len: tcp_header_len,
        flags: tcp_header_len__keep_flags[10..=15].to_string(),
        window_size: i32::from_str_radix(u8s_to_0xs(&data[14..=15]).join("").as_str(),16).unwrap(),
        checksum: u8s_to_0xs(&data[16..=17]).join(""),
        urgent_pointer: i32::from_str_radix(u8s_to_0xs(&data[18..=19]).join("").as_str(),16).unwrap(),
        options: data[20..(tcp_header_len as usize)].to_vec(),
    }
}

//解析udp头部
pub fn hand_udp_head(data:&[u8]) -> udp_head{
    udp_head{
        src_port:  i32::from_str_radix(u8s_to_0xs(&data[0..=1]).join("").as_str(),16).unwrap(),
        dst_port: i32::from_str_radix(u8s_to_0xs(&data[2..=3]).join("").as_str(),16).unwrap(),
        udp_len: i32::from_str_radix(u8s_to_0xs(&data[4..=5]).join("").as_str(),16).unwrap(),
        checksum: u8s_to_0xs(&data[6..=7]).join("")
    }
}


//解析arp
pub fn hand_arp(data:&[u8]) -> arp{
    let hardware_type = u8s_to_0xs(&data[0..=1]).join("");
    let protocol_type = u8s_to_0xs(&data[2..=3]).join("");
    let hardware_length = data[4];
    let protocol_length = data[5];
    let operation_code = u8s_to_0xs(&data[6..=7]).join("");
    let source_hardware_address = u8s_to_0xs(&data[8..=13]).join(":");
    let source_protocol_address = data[14..=17].iter().map(|x| format!("{}",x)).collect::<Vec<String>>().join(".");
    let destination_hardware_address = u8s_to_0xs(&data[18..=23]).join(":");
    let destination_protocol_address = data[24..=27].iter().map(|x| format!("{}",x)).collect::<Vec<String>>().join(".");
    arp{
        hardware_type:hardware_type,
        protocol_type:protocol_type,
        hardware_length:hardware_length,
        protocol_length:protocol_length,
        operation_code:operation_code,
        source_hardware_address:source_hardware_address,
        source_protocol_address:source_protocol_address,
        destination_hardware_address:destination_hardware_address,
        destination_protocol_address:destination_protocol_address
    }
}
