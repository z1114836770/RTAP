use std::collections::BTreeMap;
use crate::eth_2_struct::eth_2_ip_tcp;
use serde_json::value::Value::Null;

//进行ip重组
//集合 存放ip_tcp
// key为源ip和目标ip和标识位和上层协议
// val为需要重组的分片ip_tcp
pub fn ip_regroup(ip_map:&mut BTreeMap<String,Vec<eth_2_ip_tcp>>, ip_tcp:eth_2_ip_tcp) -> Option<eth_2_ip_tcp>{

    let source_address  = &ip_tcp.ip_head.source_address;
    let dst_address = &ip_tcp.ip_head.destination_address;
    let protocol = &ip_tcp.ip_head.protocol;
    let identifier = &ip_tcp.ip_head.identifier;
    let key = format!("{}{}{}{}",source_address,dst_address,identifier,protocol);

//    判断是否为分片报文，如果是独立报文直接返回该报文，如果是需要重组报文 则 进行报文重组
    let flags = &ip_tcp.ip_head.flags;

    let flags_1 = flags.chars().nth(1).unwrap();
    let flags_2 = flags.chars().nth(2).unwrap();

//    println!("{}",flags);

//    判断数据包是否分片
//    如果不分片直接返回
    if flags_1 == '1' || flags_2 == '0'{
        return Some(ip_tcp)
    }else {
//        判断 是否已经有 分片集合存在
        if ip_map.contains_key(key.as_str()) {
//            判断 分片是否已经存在对应的集合内
            if ip_map.get(key.as_str()).unwrap().contains(&ip_tcp) {
                return None
            }else{
//                将分片添加进分片集合内
                ip_map.get_mut(&key).unwrap().push(ip_tcp);
                return None
            }
        }else{
//            不存就 创建分片集合
            ip_map.insert(key,vec![ip_tcp]);
            return None
        }
    }

    None

}