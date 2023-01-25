use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use amalgam::SliceRange;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:59100").await?;

    let addr = listener.local_addr()?;
    println!("Starting TCP listener on {:?}", addr);

    loop {
        let (socket, _saddr) = listener.accept().await?;

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = vec![0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            // Return value of `Ok(0)` signifies remote has closed
            Ok(0) => {
                eprintln!("Socket read received `EOF`.");
                return;
            }
            Ok(n) => {
                // Received `n` bytes of data. Copy data back to socket.
                match socket.write_all(&buffer[..n]).await {
                    Ok(_) => {
                        // convert to slice
                        // let len = buffer.len();
                        // utf-8 encoded bytes
                        // SAFETY: All data should be UTF-8 encoded; the parser will reject invalid data.
                        println!("Read `{}` bytes. Sending to parser...", n);
                        let utf8 = unsafe { std::str::from_utf8_unchecked(&buffer[..n]) };

                        println!(
                            "Read {} bytes:\n{:?}\nUTF-8:\n{}",
                            n,
                            &buffer[..n],
                            &utf8[..n]
                        );
                    }
                    Err(err) => {
                        eprintln!("Error reading from socket: {:?}", err);
                        return;
                    }
                }
            }
            Err(err) => {
                eprintln!("Client disconnected: {:?}", err);
                return;
            }
        };
    }
}
