use tokio::net::{TcpListener, TcpStream};
use tokio::io;

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

async fn handle_connection(mut inbound: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut outbound = TcpStream::connect(UPSTREAM_ADDR).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_upstream = io::copy(&mut ri, &mut wo);
    let upstream_to_client = io::copy(&mut ro, &mut wi);

    tokio::try_join!(client_to_upstream, upstream_to_client)?;
    Ok(())
}