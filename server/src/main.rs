use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio::io::copy_bidirectional;
use std::env;
use tokio::time::{sleep, Duration};

#[derive(Deserialize)]
struct Data {
    port: i32,
}

async fn connect_handler(req: HttpRequest, json_data: web::Json<Data>) -> impl Responder {
    let port = json_data.port;
    let addr = format!("{}:{}", req.connection_info().peer_addr().unwrap(), port);
    let mut client_stream = TcpStream::connect(addr).await.unwrap();
    match env::var("MINECRAFT_SERVER") {
        Ok(minecraft_ip) => {
            sleep(Duration::from_secs(5)).await;
            let mut minecraft_stream = TcpStream::connect(minecraft_ip).await.unwrap();
            copy_bidirectional(&mut client_stream, &mut minecraft_stream).await.unwrap();
            "Done"
        }
        Err(_) => {
            "Server is not set!"
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/connect").route(web::post().to(connect_handler)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
