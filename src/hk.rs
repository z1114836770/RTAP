use std::collections::HashMap;



//判断数据是否包含扫描工具关键字
pub fn hadn_hk(info:&String, hk_map:&HashMap<String,String>){
    let mut res = String::new();
    for (k,v ) in hk_map{
        if info.contains(k) {
            res = v.clone();
            break
        }
    }
    if res.len() > 0 {
        println!("request is hk action : {}",res);
        // do
        // do
        // do
    }
}