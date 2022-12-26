use std::time::{Duration, Instant};

use hyper::{*, body::to_bytes, client::{Client, HttpConnector}};

async fn download_file(client: Client<HttpConnector, Body>, url: String) -> hyper::Result<(Duration, String, usize)> { 
    let uri = format!("{url}/index.html")
        .parse::<Uri>().unwrap();
    let req = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let now = Instant::now();
    let response = client.request(req).await;
    match response {
        Ok(body) => {
            let body = to_bytes(body.into_body()).await.unwrap();
            Ok((now.elapsed(), url, body.len()))
        }
        Err(e) => Err(e),
    }
}

#[tokio::main]
async fn main() {
    let mut set = tokio::task::JoinSet::new();
    let urls = vec![
        "http://reddit.com",
        "http://google.com",
        "http://amazon.com",
        "http://example.org",
    ];
    let mut results = Vec::with_capacity(urls.len());
    let client = Client::new();
    for i in urls.iter() {
        set.spawn(download_file(client.clone(), i.to_string()));
    }
    while let Some(res) = set.join_next().await {
        match res.unwrap() {
            Ok(res) => results.push(res),
            Err(e) => eprintln!("{e}"),
        }
    }
    results.sort();
    for (duration, url, len) in results {
        println!("{url}: {} ({len})", duration.as_millis());
    }
}
