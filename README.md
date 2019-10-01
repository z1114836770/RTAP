网络流量实时分析工具（网络流量警告工具）

目标：针对宿主机产生的流量，进行实时分析。发现  恶意扫描、攻击请求 或 向不明目标服务器定期发送敏感数据。

警告响应处理：通知机主、禁止IP

运行系统：linux

运行环境：libpcap

配置文件：config

工具运行：sudo ./pcap_rust

持续开发中 . . . . . .


已实现解析：

    Ehternet II \ IP \ TCP \ 数据         
		Ehternet II \ ARP

示例：可见示例.png


以下为www.baidu.com分片流量包数据

eth_2_ip_tcp { eth_head: eth_2_head { dst_mac: "00:0c:29:06:8e:04", src_mac: "00:50:56:e7:a5:97", data_type: "0800" }, ip_head: ipv4_head { ip_version: 4, ip_pg_head_len: 20, service_type: "00000000", ip_pg_len: 1400, identifier: "50bd", flags: "000", flags_offset: "0000000000000", time_to_live: 128, protocol: 6, header_checksum: "23ff", source_address: "183.232.231.172", destination_address: "192.168.96.134", options: [] }, tcp_head: tcp_head { src_port: 80, dst_port: 29626, sequence_number: 942766962, ack_number: 3355730768, tcp_header_len: 20, flags: "011000", window_size: 64240, checksum: "e1cb", urgent_pointer: 0, options: [] }, data_bit: [72, 84, 84, 80, 47, 49, 46, 49, 32, 50, 48, 48, 32, 79, 75, 13, 10, 65, 99, 99, 101, 112, 116, 45, 82, 97, 110, 103, 101, 115, 58, 32, 98, 121, 116, 101, 115, 13, 10, 67, 97, 99, 104, 101, 45, 67, 111, 110, 116, 114, 111, 108, 58, 32, 112, 114, 105, 118, 97, 116, 101, 44, 32, 110, 111, 45, 99, 97, 99, 104, 101, 44, 32, 110, 111, 45, 115, 116, 111, 114, 101, 44, 32, 112, 114, 111, 120, 121, 45, 114, 101, 118, 97, 108, 105, 100, 97, 116, 101, 44, 32, 110, 111, 45, 116, 114, 97, 110, 115, 102, 111, 114, 109, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58, 32, 75, 101, 101, 112, 45, 65, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 50, 51, 56, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45, 84, 121, 112, 101, 58, 32, 116, 101, 120, 116, 47, 104, 116, 109, 108, 13, 10, 68, 97, 116, 101, 58, 32, 83, 97, 116, 44, 32, 50, 56, 32, 83, 101, 112, 32, 50, 48, 49, 57, 32, 48, 51, 58, 53, 51, 58, 53, 50, 32, 71, 77, 84, 13, 10, 69, 116, 97, 103, 58, 32, 34, 53, 56, 56, 54, 48, 52, 101, 99, 45, 57, 52, 100, 34, 13, 10, 76, 97, 115, 116, 45, 77, 111, 100, 105, 102, 105, 101, 100, 58, 32, 77, 111, 110, 44, 32, 50, 51, 32, 74, 97, 110, 32, 50, 48, 49, 55, 32, 49, 51, 58, 50, 56, 58, 49, 50, 32, 71, 77, 84, 13, 10, 80, 114, 97, 103, 109, 97, 58, 32, 110, 111, 45, 99, 97, 99, 104, 101, 13, 10, 83, 101, 114, 118, 101, 114, 58, 32, 98, 102, 101, 47, 49, 46, 48, 46, 56, 46, 49, 56, 13, 10, 83, 101, 116, 45, 67, 111, 111, 107, 105, 101, 58, 32, 66, 68, 79, 82, 90, 61, 50, 55, 51, 49, 53, 59, 32, 109, 97, 120, 45, 97, 103, 101, 61, 56, 54, 52, 48, 48, 59, 32, 100, 111, 109, 97, 105, 110, 61, 46, 98, 97, 105, 100, 117, 46, 99, 111, 109, 59, 32, 112, 97, 116, 104, 61, 47, 13, 10, 13, 10, 60, 33, 68, 79, 67, 84, 89, 80, 69, 32, 104, 116, 109, 108, 62, 13, 10, 60, 33, 45, 45, 83, 84, 65, 84, 85, 83, 32, 79, 75, 45, 45, 62, 60, 104, 116, 109, 108, 62, 32, 60, 104, 101, 97, 100, 62, 60, 109, 101, 116, 97, 32, 104, 116, 116, 112, 45, 101, 113, 117, 105, 118, 61, 99, 111, 110, 116, 101, 110, 116, 45, 116, 121, 112, 101, 32, 99, 111, 110, 116, 101, 110, 116, 61, 116, 101, 120, 116, 47, 104, 116, 109, 108, 59, 99, 104, 97, 114, 115, 101, 116, 61, 117, 116, 102, 45, 56, 62, 60, 109, 101, 116, 97, 32, 104, 116, 116, 112, 45, 101, 113, 117, 105, 118, 61, 88, 45, 85, 65, 45, 67, 111, 109, 112, 97, 116, 105, 98, 108, 101, 32, 99, 111, 110, 116, 101, 110, 116, 61, 73, 69, 61, 69, 100, 103, 101, 62, 60, 109, 101, 116, 97, 32, 99, 111, 110, 116, 101, 110, 116, 61, 97, 108, 119, 97, 121, 115, 32, 110, 97, 109, 101, 61, 114, 101, 102, 101, 114, 114, 101, 114, 62, 60, 108, 105, 110, 107, 32, 114, 101, 108, 61, 115, 116, 121, 108, 101, 115, 104, 101, 101, 116, 32, 116, 121, 112, 101, 61, 116, 101, 120, 116, 47, 99, 115, 115, 32, 104, 114, 101, 102, 61, 104, 116, 116, 112, 58, 47, 47, 115, 49, 46, 98, 100, 115, 116, 97, 116, 105, 99, 46, 99, 111, 109, 47, 114, 47, 119, 119, 119, 47, 99, 97, 99, 104, 101, 47, 98, 100, 111, 114, 122, 47, 98, 97, 105, 100, 117, 46, 109, 105, 110, 46, 99, 115, 115, 62, 60, 116, 105, 116, 108, 101, 62, 231, 153, 190, 229, 186, 166, 228, 184, 128, 228, 184, 139, 239, 188, 140, 228, 189, 160, 229, 176, 177, 231, 159, 165, 233, 129, 147, 60, 47, 116, 105, 116, 108, 101, 62, 60, 47, 104, 101, 97, 100, 62, 32, 60, 98, 111, 100, 121, 32, 108, 105, 110, 107, 61, 35, 48, 48, 48, 48, 99, 99, 62, 32, 60, 100, 105, 118, 32, 105, 100, 61, 119, 114, 97, 112, 112, 101, 114, 62, 32, 60, 100, 105, 118, 32, 105, 100, 61, 104, 101, 97, 100, 62, 32, 60, 100, 105, 118, 32, 99, 108, 97, 115, 115, 61, 104, 101, 97, 100, 95, 119, 114, 97, 112, 112, 101, 114, 62, 32, 60, 100, 105, 118, 32, 99, 108, 97, 115, 115, 61, 115, 95, 102, 111, 114, 109, 62, 32, 60, 100, 105, 118, 32, 99, 108, 97, 115, 115, 61, 115, 95, 102, 111, 114, 109, 95, 119, 114, 97, 112, 112, 101, 114, 62, 32, 60, 100, 105, 118, 32, 105, 100, 61, 108, 103, 62, 32, 60, 105, 109, 103, 32, 104, 105, 100, 101, 102, 111, 99, 117, 115, 61, 116, 114, 117, 101, 32, 115, 114, 99, 61, 47, 47, 119, 119, 119, 46, 98, 97, 105, 100, 117, 46, 99, 111, 109, 47, 105, 109, 103, 47, 98, 100, 95, 108, 111, 103, 111, 49, 46, 112, 110, 103, 32, 119, 105, 100, 116, 104, 61, 50, 55, 48, 32, 104, 101, 105, 103, 104, 116, 61, 49, 50, 57, 62, 32, 60, 47, 100, 105, 118, 62, 32, 60, 102, 111, 114, 109, 32, 105, 100, 61, 102, 111, 114, 109, 32, 110, 97, 109, 101, 61, 102, 32, 97, 99, 116, 105, 111, 110, 61, 47, 47, 119, 119, 119, 46, 98, 97, 105, 100, 117, 46, 99, 111, 109, 47, 115, 32, 99, 108, 97, 115, 115, 61, 102, 109, 62, 32, 60, 105, 110, 112, 117, 116, 32, 116, 121, 112, 101, 61, 104, 105, 100, 100, 101, 110, 32, 110, 97, 109, 101, 61, 98, 100, 111, 114, 122, 95, 99, 111, 109, 101, 32, 118, 97, 108, 117, 101, 61, 49, 62, 32, 60, 105, 110, 112, 117, 116, 32, 116, 121, 112, 101, 61, 104, 105, 100, 100, 101, 110, 32, 110, 97, 109, 101, 61, 105, 101, 32, 118, 97, 108, 117, 101, 61, 117, 116, 102, 45, 56, 62, 32, 60, 105, 110, 112, 117, 116, 32, 116, 121, 112, 101, 61, 104, 105, 100, 100, 101, 110, 32, 110, 97, 109, 101, 61, 102, 32, 118, 97, 108, 117, 101, 61, 56, 62, 32, 60, 105, 110, 112, 117, 116, 32, 116, 121, 112, 101, 61, 104, 105, 100, 100, 101, 110, 32, 110, 97, 109, 101, 61, 114, 115, 118, 95, 98, 112, 32, 118, 97, 108, 117, 101, 61, 49, 62, 32, 60, 105, 110, 112, 117, 116, 32, 116, 121, 112, 101, 61, 104, 105, 100, 100, 101, 110, 32, 110, 97, 109, 101, 61, 114, 115, 118, 95, 105, 100, 120, 32, 118, 97, 108, 117, 101, 61, 49, 62, 32, 60, 105, 110, 112, 117, 116, 32, 116, 121, 112, 101, 61, 104, 105, 100, 100, 101, 110, 32, 110, 97, 109, 101, 61, 116, 110, 32, 118, 97, 108, 117, 101, 61, 98, 97, 105, 100, 117, 62, 60, 115, 112, 97, 110, 32, 99, 108, 97, 115, 115, 61, 34, 98, 103, 32, 115, 95, 105, 112, 116, 95, 119, 114, 34, 62, 60, 105, 110, 112, 117, 116, 32, 105, 100, 61, 107, 119, 32, 110, 97, 109, 101, 61, 119, 100, 32, 99, 108, 97, 115, 115, 61, 115, 95, 105, 112, 116, 32, 118, 97, 108, 117, 101, 32, 109, 97, 120, 108, 101, 110, 103, 116, 104, 61, 50, 53, 53, 32, 97, 117, 116, 111, 99, 111, 109, 112, 108, 101, 116, 101, 61, 111, 102, 102, 32, 97, 117, 116, 111, 102, 111, 99, 117, 115, 62, 60, 47], data_info: "HTTP/1.1 200 OK\r\nAccept-Ranges: bytes\r\nCache-Control: private, no-cache, no-store, proxy-revalidate, no-transform\r\nConnection: Keep-Alive\r\nContent-Length: 2381\r\nContent-Type: text/html\r\nDate: Sat, 28 Sep 2019 03:53:52 GMT\r\nEtag: "588604ec-94d"\r\nLast-Modified: Mon, 23 Jan 2017 13:28:12 GMT\r\nPragma: no-cache\r\nServer: bfe/1.0.8.18\r\nSet-Cookie: BDORZ=27315; max-age=86400; domain=.baidu.com; path=/\r\n\r\n\r\n <meta http-equiv=content-type content=text/html;charset=utf-8><meta http-equiv=X-UA-Compatible content=IE=Edge><title>百度一下，你就知道</title>


<span class="bg s_ipt_wr"></" }
持续更新......

