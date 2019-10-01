
pub fn u8_to_bit(data:&u8) -> String{
    format!("{number:0>width$}", number=format!("{:b}",data), width=8)
}

pub fn u8_to_0x(data:u8) -> String{
    format!("{number:0>width$}", number=format!("{:x}",data), width=2)
}

pub fn u8s_to_0xs(data:&[u8]) -> Vec<String>{
    data.iter()
        .map(|x| format!("{:x}",x))
        .map(|x| format!("{:0>2}",x))
        .collect::<Vec<String>>()
}