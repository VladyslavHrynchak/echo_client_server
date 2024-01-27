
use tokio::{net::TcpStream, io::AsyncWriteExt, io::AsyncReadExt};
use std::io;

const ECHO_SERVER_ADDRESS: &str = "localhost:8000";

#[tokio::main]
async fn main() {
    println!("Connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await{
        println!("Connected to {}::{}",
         stream.local_addr().unwrap().ip(),
         stream.local_addr().unwrap().port());

         loop{
            let mut message = String::new();
            io::stdin().read_line(&mut message).unwrap();

            stream.write_all(message.as_bytes()).await.unwrap();
            println!("sent: {message}");
   
           let mut buffer = [0; 1024];
            stream.read(&mut buffer).await.unwrap();

           let message = String::from_utf8_lossy(& mut buffer);
           println!("recieved: {}", message);
        }
    }
    else {
        println!("Failed to connect to the server");
    }
}
