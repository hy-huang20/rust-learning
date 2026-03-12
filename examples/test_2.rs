use tokio_async_crawler::*;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url_vec = Vec::<String>::new();
    let origin_url: &str = "https://search.douban.com/book/subject_search?search_text=rust&start=";
    url_gen(&mut url_vec,origin_url);
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let client = Client::builder().build().unwrap();
    let handles = url_vec.into_iter().map(|url| tokio::spawn(handle_url(url))).collect::<Vec<_>>();
    let results = join_all(handles).await;
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("cost time is {:?}",end_time-start_time);
    return Ok(());
}