use crate::tools::u8_to_0x;

pub enum eth_format{
    ETHERNET_II,
    NOVELL_ETHERNET,
    IEEE_802_3_SAP,
    IEEE_802_3_SNAP,
    UNKNOWN,
}

impl eth_format{

//    判断以太帧格式
    pub fn hand_eth_format(data:&[u8]) -> eth_format{
        let data_type = [u8_to_0x(data[12]),u8_to_0x(data[13])].join("");
        let data_lable1 = &data[14];
        let data_lable2 = &data[15];

        // FF = 255
        if *data_lable1 == 255 as u8 && *data_lable2 == 255 as u8{
            eth_format::NOVELL_ETHERNET
            // AA = 170
        }else if  *data_lable1 == 170 as u8 && *data_lable2 == 170 as u8{
            eth_format::IEEE_802_3_SNAP
        }
        else if let Ok(x) = i16::from_str_radix(data_type.as_str(), 16){
            // 05ff = 1535
            if x > 1500{
                eth_format::ETHERNET_II
            }else {
                eth_format::IEEE_802_3_SAP
            }
        }else{
            eth_format::UNKNOWN
        }
    }
}