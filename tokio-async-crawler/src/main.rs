use std::{any::Any, time::{SystemTime, UNIX_EPOCH}};

use reqwest::Client;
use json;
use tokio::join;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url_vec = Vec::<String>::new();
    let origin_url: &str = "https://search.douban.com/book/subject_search?search_text=rust&start=";
    url_gen(&mut url_vec,origin_url);
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let client = Client::builder().build().unwrap();
    let mut handles = FuturesUnordered::new();
    for url in url_vec {
        handles.push(tokio::spawn(handle_url(url)));
    }
    while let Some(result) = handles.next().await {}
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("cost time is {:?}",end_time-start_time);
    return Ok(());
}

fn url_gen(url_vec:&mut Vec<String>,origin_url:&str){    
    let step = 15;
    for index in (0..10){
        let start = index*step;
        let url = origin_url.to_string()+&start.to_string();
        url_vec.push(url);
    }

}

async fn handle_url(url:String){
    println!("handle url {} start",&url);            
    let html = reqwest::get(&url).await.unwrap().text().await.unwrap();        
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
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