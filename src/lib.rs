use monoio::net::TcpListener;

pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:50002").unwrap();
    println!("listening");
    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((_stream, addr)) => {
                println!("accepted a connection from {}", addr);
            }
            Err(e) => {
                println!("accepted connection failed: {}", e);
                return;
            }
        }
    }
}