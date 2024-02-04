use reqwest::Client;
use tokio::net::TcpListener;
use tokio::io::copy_bidirectional;
use serde_json::json;
use std::time::Duration;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <http api ip/port>", args[0]);
        return Ok(());
    }

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Waiting on 127.0.0.1:8080");
    let mut socket = listener.accept().await?.0;

    let mc_listener = TcpListener::bind("0.0.0.0:0").await?;

    let port = mc_listener.local_addr().unwrap().port();

    let client = Client::builder().timeout(Duration::from_secs(1)).build()?;
    let _ = client.post(format!("{}/connect", args[1]))
        .json(&json!({"port": port}))
        .send()
        .await;
    let mut mc_socket = mc_listener.accept().await?.0;

    copy_bidirectional(&mut socket, &mut mc_socket).await?;
    loop{}
    Ok(())
}
