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



pub fn config_info() -> (config,HashMap<String,String>){

    let mut config_info = String::new();

    
    match std::fs::File::open("config") {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(l) => {
                        let line = l;
                        if !line.contains("#") && line.replace(" ","").len() != 0 {
                            config_info.push_str(line.replace(" ","").as_str());
                        }
                    }
                    Err(e) =>{
                        println!("read config line er");
                        exit(0);
                    }
                }

            }
        }
        Err(e) => {
            println!("not read config file");
            exit(0);
        }
    }

//    for line in BufReader::new(std::fs::File::open("config").expect("not read config file")).lines() {
//        let line = line.expect("read config line err");
//        if !line.contains("#") && line.replace(" ","").len() != 0 {
//            config_info.push_str(line.replace(" ","").as_str());
//        }
//    }


    let mut con:config ;

    let a = serde_json::de::from_str::<config>(config_info.as_str());
    match a {
        Ok(j) => {
            con = j;
        }
        Err(e) => {
            println!("format json err");
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

//    let con: config = serde_json::de::from_str(config_info.as_str()).expect("json err");


    let mut hk:HashMap<String,String> = HashMap::new();
    for hks_key in &con.hks_key{
        for key in &hks_key.keys{
            hk.insert(key.clone(),hks_key.name.clone());
        }
    }
    (con,hk)
}