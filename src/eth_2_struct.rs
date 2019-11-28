use std::collections::HashMap;

#[derive(Debug)]
pub enum eth_2_struct{
    ETH_IP_TCP(eth_2_ip_tcp),
    ETH_IP_UDP(eth_2_ip_udp),
    ARP(arp),
    UNKOWN(String)
}

#[derive(Debug, PartialEq)]
pub struct eth_2_ip_tcp{
    pub eth_head:eth_2_head,
    pub ip_head:ipv4_head,
    pub tcp_head:tcp_head,
    pub data_bit:Vec<u8>,
    pub data_info:String,
}

#[derive(Debug)]
pub struct eth_2_ip_udp{
    pub eth_head:eth_2_head,
    pub ip_head:ipv4_head,
    pub udp_head:udp_head,
    pub data_bit:Vec<u8>,
    pub data_info:String,
}

#[derive(Debug, PartialEq)]
pub struct eth_2_head{
    //  目标mac地址  6字节 返回：十六进制数组
    pub dst_mac:String,
    //  源mac地址  6字节 返回：十六进制数组
    pub src_mac:String,
    //   协议类型  2字节  返回：十六进制
    pub data_type:String
}

//arp头部
#[derive(Debug)]
pub struct arp{
    //硬件地址类型，一般为以太网；  2字节
    pub hardware_type:String,
    //：表示三层协议地址类型，一般为IP；    2字节
    pub protocol_type:String,
    //MAC地址长度，单位是字节     1字节
    pub hardware_length:u8,
    //IP地址的长度，单位是字节；    1字节
    pub protocol_length:u8,
    //指定了ARP报文的类型，包括ARP request和ARP reply；   2字节
    pub operation_code:String,
    // 指的是发送ARP报文的设备MAC地址；      6字节
    pub source_hardware_address:String,
    //指的是发送ARP报文的设备IP地址；        4字节
    pub source_protocol_address:String,
    //指的是接收者MAC地址，在ARP request报文中，该字段值为0；       6字节
    pub destination_hardware_address:String,
    //指的是接受者的IP地址。      4字节
    pub destination_protocol_address:String,
}


//ipv4头部
#[derive(Debug, PartialEq)]
pub struct ipv4_head{

//    ip协议  占4位  返回：十进制
    pub ip_version:i32,

//    ip头部长度  占4位  返回：十进制
    pub ip_pg_head_len:i32,

//    服务类型  占8位  返回：二进制
    pub service_type:String,

//    ip包总长度 16位  返回：十进制
    pub ip_pg_len:i32,

//    标识符 16位 返回：十六进制
    pub identifier:String,

//    标记  3位 返回：二进制
    pub flags:String,

//    片便宜 13位 返回：二进制
    pub flags_offset:String,

//    生存时间 8位 返回：十进制
    pub time_to_live:i32,

//    ip数据包内数据类型  8位  返回：十进制
    pub protocol:i32,

//    校验和  16位  返回：十六进制
    pub header_checksum:String,

//    源地址  32位 返回：十进制数组
    pub source_address:String,

//    目标地址  32位  返回：十进数组
    pub destination_address:String,

//    可选项  返回：十进制数组
    pub options:Vec<u8>,
}

//IGMP v1
pub struct igmpv1_head{
//    IGMP版本号  4位  返回：十六进制
    pub version:String,
//    IGMP报文类型  4位   返回：十六进制
//1 = Host Membership Query 主机成员查询
//2 = Host Membership Report 主机成员报告
    pub igmp_type:String,
//    未使用的字段，发送时必须填0，接收时忽略。     8位  返回：二进制
    pub unused: String,
//      IGMP消息的校验和。
//      该字段在进行校验计算时设为0。
//      当传送报文的时候，
//      必须计算该校验字并插入到该字段中去。
//      当接收包的时候，
//      该校验字必须在处理该包之前进行检验。
//      16位   返回：十六进制
    pub checksum:String,
//      组播地址（ip地址）  32位  返回
    pub group_address:String
}
//IGMP v2
pub struct igmpv2_head{
//    报文类型，有以下几种类型：
//    0x11 = Membership Query IGMP查询消息。
//    0x12 = Version 1 Membership Report IGMPv1成员报告消息。
//    0x16 = Version 2 Membership Report IGMPv2成员报告消息。
//    0x17 = Leave Group 离开消息。
//    在IGMP版本2中，旧的4位版本字段和旧的4位类型字段拼成了一个新的8位类型字段，通过分别将成员查询（版本1和版本2的）及版本1的成员报告报文的IGMP版本2的类型代码置为0x11和0x12，保持了IGMP版本1和版本2包格式的向后兼容。
//    8位  返回：十六禁止
    pub igmp_type:String,

//    在发出响应报告前的以1/10秒为单位的最长时间，缺省值为10秒。
//    新的最大响应时间（以1/10秒为单位）字段允许查询用路由器为它的查询报文指定准确的查询间隔响应时间。IGMP版本2主机在随机选择它们的响应时间值时以此作为上限。
//    这样在查询响应间隔时有助于控制响应的爆发。
//      8比特  返回：十进制
    pub max_resp_time:i32,

//    IGMP消息的校验和。
//      传送报文时，必须计算校验和并填入该字段中；
//      接收报文时，必须在处理报文之前检验校验和，
//      以判断IGMP消息在传输过程中是否发生了错误。
//      16位   返回：十六进制
    pub checksum:String,
    //      组播地址（ip地址）  32位  返回
    pub group_address:String
}
//IGMP v3
pub struct igmpv3_head{

}


//tcp头部
#[derive(Debug, PartialEq)]
pub struct tcp_head{

//    源端口号  16位  返回：十进制
    pub src_port:i32,

//    目标端口号  16位  返回：十进制
    pub dst_port:i32,

//    序号  32位  返回：十进制
    pub sequence_number:i64,

//    确认号  32位  返回：十进制
    pub ack_number:i64,

//    tcp头部长度  4位  返回：十进制
    pub tcp_header_len:i32,

//    保留6位

//    标示  6位  返回：二进制
    pub flags:String,

//    窗口大小   16位  返回：十进制
    pub window_size:i32,

//    校验和  16位  返回：十六进制
    pub checksum:String,

//    紧急指针  16位  返回：十进制
    pub urgent_pointer:i32,

//    可选项
    pub options:Vec<u8>,
}

//tcp头加数据
#[derive(Debug, PartialEq)]
pub struct ipv4_tcp_data{
    pub ip_head:ipv4_head,
    pub tcp_head:tcp_head,
    pub tcp_data:Vec<u8>
}

//重组tcp时需要的结构体
#[derive(Debug, PartialEq)]
pub struct tcp_regroup_struct{
//    tcp包中数据部分总长度
    pub all_tcp_data_len:i32,
//    tcp重组结构创建时间
    pub create_tcp_regroup_time:i64,
//    新入tcp包时间
    pub new_in_tcp_time:i64,
//    结束位标示（最后一个数据包是否已经到，但是中间还没有到。1表示已到，往后再到的数据包 需要进行结束合并计算）
    pub end_lable:i32,
//    第一个请求的tcp包
    pub syn_seq:tcp_head,
//    最后一个到达的tcp包
    pub fin_seq:tcp_head,

//   发送的 需要被重组的tcp包集合
    pub send_tcps:Vec<ipv4_tcp_data>,
//   接受的 需要被重组的tcp包集合
    pub res_tcps:Vec<ipv4_tcp_data>
}


//udp头部
#[derive(Debug, PartialEq)]
pub struct udp_head{
    //    源端口号  16位  返回：十进制
    pub src_port:i32,

    //    目标端口号  16位  返回：十进制
    pub dst_port:i32,

    //    tcp头部长度  16位  返回：十进制
    pub udp_len:i32,

    //    校验和  16位  返回：十六进制
    pub checksum:String,
}


//tcp重组_会话
#[derive(Debug, PartialEq)]
pub struct tcp_session{
//    源ip
    pub src_ip:String,
//    源端口
    pub src_port:i32,
//    目标ip
    pub des_ip:String,
//    目标端口
    pub des_port:i32,
//    创建时间
    pub create_time:i64,
//    更新时间
    pub update_time:i64,

//    当前客户端发送数据包长度
    pub client_data_len:i64,
//    客户端第一个包的序列号
    pub client_syn_seq:i64,
//    客户端最后一包的seq
    pub client_fin_seq:i64,
//    客户端最后一个包是否已到
    pub client_fin:bool,
    //   客户端数据是否存盘
    pub client_save:bool,
//    客户端数据包是否已全到
    pub client_data_down:bool,
//    客户端发送的tcp包链表
    pub client_nods:HashMap<i64,tcp_data>,

    //    当前服务端发送数据包长度
    pub server_data_len:i64,
    //    服务端第一个包的序列号
    pub server_syn_seq:i64,
    //    服务端最后一包的seq
    pub server_fin_seq:i64,
    //    服务端最后一个包是否已到
    pub server_fin:bool,
    //   服务端数据是否存盘
    pub server_save:bool,
    //    服务端数据包是否已全到
    pub server_data_down:bool,
//    接收的tcp包链表
    pub server_nods:HashMap<i64,tcp_data>,
}


//tcp重组_节点
#[derive(Debug, PartialEq)]
pub struct tcp_data{
//    是否未发送包
//    pub syn:i32,
////    是否未结束包
//    pub fin:i32,
////    数据包序列号
//    pub seq:i64,
////    数据包的长度
//    pub len:i32,
//    上一个tcp节点
//    pub prev_tcp_node:Option<Box<tcp_node>>,
//    下一个tcp节点
//    pub netx_tcp_node:Option<Box<tcp_node>>,
//   数据是否存盘
//    pub save:bool,
//    当前数据包长度
    pub data_len:i32,
//    数据包内容
    pub data:Vec<u8>,
//      存盘的索引
    pub data_lib:String
}





//重组后的结构
#[derive(Debug, PartialEq, Clone)]
pub struct ip_tcp_data{
    pub s_ip:String,
    pub s_port:i32,
    pub d_ip:String,
    pub d_port:i32,
    pub client_data_bit:Vec<u8>,
    pub client_data_lib:String,
    pub server_data_bit:Vec<u8>,
    pub server_data_lib:String,
}





