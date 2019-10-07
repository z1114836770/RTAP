extern crate serde_json;
extern crate serde;
//#[macro_use]
//extern crate serde_derive;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::io::BufReader;
use std::collections::HashMap;
use std::io::BufRead;
use std::process::exit;
use crate::error_log::err_log;


#[derive(Debug, Serialize, Deserialize)]
pub struct hks{
    pub name:String,
    pub keys:Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[warn(unreachable_code)]
pub struct config{
//    邮件服务器地址
    pub smtp_address:String,
//    发送邮件的账号
    pub smtp_from_username:String,
//    发送邮件的密码
    pub smtp_from_password:String,
//    接收邮件的地址
    pub smtp_to_usernames:Vec<String>,

//    监控的网卡
    pub net:String,
//    不监控源IP
    pub not_check_source_ip:Vec<String>,
//    不监控源端口
    pub not_check_source_port:Vec<String>,
//    不监控目标IP
    pub not_check_dst_ip:Vec<String>,
//    不监控目标端口
    pub not_check_dst_port:Vec<String>,
//    扫描工具关键字
    pub hks_key:Vec<hks>

}


//获取配置文件config中的内容
pub  fn config_info() -> (config,HashMap<String,String>){

//    将config中的内容读取到config_info中
    let mut config_info = String::new();

    match std::fs::File::open("config") {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(l) => {
                        let line = l;
//                        开头为#的内容为朱姐注释
                        if !line.contains("#") && line.replace(" ","").len() != 0 {
                            config_info.push_str(line.replace(" ","").as_str());
                        }
                    }
                    Err(e) =>{
                        println!("读取config内容出错，请联系开发人员");
                        err_log("读取config内容出错，请联系开发人员");
                        exit(0);
                    }
                }
            }
        }
        Err(e) => {
            println!("没有读取到config配置文件，请将config和运行程序至于同级目录");
            err_log("没有读取到config配置文件，请将config和运行程序至于同级目录");
            exit(0);
        }
    }


//    将配置内容转换成json
    let  con:config ;
    let a = serde_json::de::from_str::<config>(config_info.as_str());
    match a {
        Ok(j) => {
            con = j;
        }
        Err(e) => {
            println!("将config转成json失败");
            err_log("将config转成json失败");
            exit(0);
            con = config{
                smtp_address: "".to_string(),
                smtp_from_username: "".to_string(),
                smtp_from_password: "".to_string(),
                smtp_to_usernames: vec![],
                net: "".to_string(),
                not_check_source_ip: vec![],
                not_check_source_port: vec![],
                not_check_dst_ip: vec![],
                not_check_dst_port: vec![],
                hks_key: vec![]
            };
        }
    }

//ip\tcp 数据中的关键字
    let mut hk:HashMap<String,String> = HashMap::new();
    for hks_key in &con.hks_key{
        for key in &hks_key.keys{
            hk.insert(key.clone(),hks_key.name.clone());
        }
    }
    (con,hk)
}