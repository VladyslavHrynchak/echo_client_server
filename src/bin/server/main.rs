use tokio::{net::TcpListener, net::TcpStream, io::AsyncWriteExt, io::AsyncReadExt};

const SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    println!("setver starting {}", SERVER_ADDRESS);

    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    println!("server listening: {}", SERVER_ADDRESS);

    loop{
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });  
    }
}

async fn handle_connection(mut stream: TcpStream){
    loop{
        let mut buf = [0; 1024];

        let byte_num = stream.read(&mut buf).await.unwrap();
        if byte_num != 0
        {
            let message = String::from_utf8_lossy(&buf);
            println!("recieved: {}", message);
        
            stream.write(message.as_bytes()).await.unwrap();
        } 
    }
}
