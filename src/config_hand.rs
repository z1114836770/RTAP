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


#[derive(Debug, Serialize, Deserialize)]
pub struct hks{
    pub name:String,
    pub keys:Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct config{
    pub net:String,
    pub not_check_source_ip:Vec<String>,
    pub not_check_source_port:Vec<String>,
    pub not_check_dst_ip:Vec<String>,
    pub not_check_dst_port:Vec<String>,
    pub hks_key:Vec<hks>
}


//获取配置文件config中的内容
pub fn config_info() -> (config,HashMap<String,String>){

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
                        exit(0);
                    }
                }
            }
        }
        Err(e) => {
            println!("没有读取到config配置文件，请将config和运行程序至于同级目录");
            exit(0);
        }
    }


//    将配置内容转换成json
    let mut con:config ;
    let a = serde_json::de::from_str::<config>(config_info.as_str());
    match a {
        Ok(j) => {
            con = j;
        }
        Err(e) => {
            println!("将config转成json失败");
            exit(0);
            con = config{
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