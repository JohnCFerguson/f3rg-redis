use bytes::BytesMut;
use f3rg_redis::{helper::buffer_to_array, Command, Db};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    signal,
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let shutdown = signal::ctrl_c();
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let mut db = Db::new();
    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buf).await?;
        println!("buffer {:?}", buf);
        println!("connection accepted {:?}", socket);
        let attrs = buffer_to_array(&mut buf);
        let command = Command::get_command(&attrs[0]);
        process_query(command, attrs, &mut socket, &mut db).await?;
    }
    Ok(())
}

async fn process_query(
    command: Command,
    attrs: Vec<String>,
    socket: &mut TcpStream,
    db: &mut Db,
) -> std::io::Result<()> {
    match command {
        Command::Get => {
            let result = db.read(&attrs);
            match result {
                Ok(result) => {
                    socket.write_all(&result).await?;
                }
                Err(_err) => {
                    println!("no key found {:?}", _err);
                    socket.write_all(b"").await?;
                }
            }
            Ok(())
        }
        Command::Set => {
            let resp = db.write(&attrs);

            match resp {
                Ok(result) => {
                    println!("set result: {}", result);
                    socket.write_all(&result.as_bytes()).await?;
                }
                Err(_err) => {
                    socket.write_all(b"").await?;
                }
            }
            Ok(())
        }
        Command::Invalid => Ok(()),
    }
}
