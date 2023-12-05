use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt};
use std::io;
use dosei_proto::CronJob;

async fn process_socket(mut socket: TcpStream) -> io::Result<()> {
    let mut buf = vec![0; 1024]; // buffer for reading data

    // Read data into buffer
    let n = socket.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }
    println!("{}", n);
    // Try to deserialize the data into ClientData
    let received_data = serde_json::from_slice::<CronJob>(&buf[..n])?;
    println!("Received: {:?}", received_data); // Log the received data

    Ok(())
}

pub(crate) async fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8844").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await?;
    }
}
