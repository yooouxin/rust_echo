use async_std::io::prelude::BufReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::stream::StreamExt;
use async_std::{io::BufReader, task};
use log::{info, Level};
use std::net::SocketAddr;
use std::str::FromStr;
use async_std::io::WriteExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn echo_loop(stream : TcpStream) ->Result<()>{
    let (mut reader,mut writer) = &mut (&stream,&stream);
    info!("start echo task {} for remote {}",task::current().id(),stream.peer_addr()?);
    let mut reader = BufReader::new(&mut reader);
    loop {
        let mut string = String::new();
        if reader.read_line(&mut string).await? > 0 {
            writer.write_all(string.as_bytes()).await?;
        }else{
            info!("stop echo task {} because connection broken",task::current().id());
            break;
        }
    }
    Ok(())
}

async fn accept_loop(address: SocketAddr) -> Result<()> {
    info!("start listening on {}", address);
    let listener = TcpListener::bind(address).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let  stream = stream?;
        task::spawn(echo_loop(stream));
    }
    Ok(())
}

fn main() -> Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();
    task::block_on(accept_loop(
        SocketAddr::from_str("127.0.0.1:56432").unwrap(),
    ))
}
