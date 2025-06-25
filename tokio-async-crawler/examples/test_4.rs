use tokio_async_crawler::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url_vec = Vec::<String>::new();
    let origin_url: &str = "https://search.douban.com/book/subject_search?search_text=rust&start=";
    url_gen(&mut url_vec,origin_url);
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let client = Client::builder().build().unwrap();
    let mini_tokio = MiniTokio::new();
    for url in url_vec {
        mini_tokio.spawn(handle_url(url));
    }
    // 运行 executor
    mini_tokio.run();
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("cost time is {:?}",end_time-start_time);
    return Ok(());
}