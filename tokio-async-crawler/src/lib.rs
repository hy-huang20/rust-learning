pub use std::{any::Any, time::{SystemTime, UNIX_EPOCH}};

pub use reqwest::Client;
pub use json;
pub use tokio::join;

mod executor;
pub use executor::MiniTokio;

///
pub fn url_gen(url_vec:&mut Vec<String>,origin_url:&str){    
    let step = 15;
    for index in (0..10){
        let start = index*step;
        let url = origin_url.to_string()+&start.to_string();
        url_vec.push(url);
    }
}

///
pub async fn handle_url(url:String){
    println!("handle url {} start",&url);            
    let html = reqwest::get(&url).await.unwrap().text().await.unwrap();        
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    let index_wind = html.find(" window.__DATA__").unwrap();
    let index_user = html.find(" window.__USER__").unwrap();
    let text = &html[index_wind+19..index_user-7];
    let data = json::parse(text).unwrap();
    let (name,mut entries) = data.entries().find(|(name,value)| *name == "items").unwrap();
    let mut index = 0;
    for data in entries.members(){
        print!("摘要：{},",data["abstract"].as_str().unwrap());
        println!("链接:{}",data["url"].as_str().unwrap());
    }    
    println!("handle url {} end",url); 
}