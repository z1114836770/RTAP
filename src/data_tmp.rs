use std::io::{Write, Read};

//将数据写到临时文件内
pub fn write_data_tmp(tmp_file_name: &String, data:&[u8]){
    std::fs::File::create(format!("{}{}","/home/zbf/zbf_tmp/" , tmp_file_name)).unwrap().write_all(data);
}

//将数据冲临时文件内读取
pub fn read_data_tmp(tmp_file_name: &String) -> Vec<u8>{
    let mut res_data = vec![];
    std::fs::File::open(format!("{}{}","/home/zbf/zbf_tmp/" , tmp_file_name)).unwrap().read_to_end(&mut res_data);
    res_data
}
