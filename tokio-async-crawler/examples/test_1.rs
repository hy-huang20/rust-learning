use tokio_async_crawler::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url_vec = Vec::<String>::new();
    let origin_url: &str = "https://search.douban.com/book/subject_search?search_text=rust&start=";
    url_gen(&mut url_vec,origin_url);
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let client = Client::builder().build().unwrap();
    let mut handles = Vec::with_capacity(10);
    for url in url_vec{
        handles.push(tokio::spawn(handle_url(url)));
    }
    for handle in handles {
        handle.await; // 而且每个 handle 必须执行完才能执行下一个 handle
    }
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("cost time is {:?}",end_time-start_time);
    return Ok(());
}