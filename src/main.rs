use tokio::net::{TcpListener, TcpStream};
use hyper::{Request, Response};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use http_body_util::{BodyExt, Full};
use bytes::Bytes;

const UPSTREAM_ADDR: &str = "127.0.0.1:19876";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server listening on port 8080");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("New connection from {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let io = TokioIo::new(stream);
    http1::Builder::new()
        .serve_connection(io, service_fn(proxy_request))
        .await?;
    Ok(())
}

async fn proxy_request(
    req: Request<Incoming>
) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    // Connect to UPSTREAM_ADDR
    let upstream = TcpStream::connect(UPSTREAM_ADDR).await?;
    let io = TokioIo::new(upstream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    let req = req.map(|body| body.boxed());
    let resp = sender.send_request(req).await?;
    let (parts, incoming) = resp.into_parts();
    let body = incoming.collect().await?.to_bytes();
    Ok(Response::from_parts(parts, Full::new(body)))
}