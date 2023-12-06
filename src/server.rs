use crate::Handler;
use crate::Listener;

pub async fn run(listener: &Listener) -> std::io::Result<()> {
    loop {
        let socket = listener.accept().await?;
        let mut handler = Handler::new(listener, socket);

        tokio::spawn(async move {
            if let Err(_err) = process_method(&mut handler).await {
                println!("Connection Error");
            }
        });
    }
}

async fn process_method(handler: &mut Handler) -> Result<(), std::io::Error> {
    while !handler.shutdown.is_shutdown() {
        let result = tokio::select! {
            _ = handler.shutdown.listen_recv() => {
                return Ok(());
            },
            res = handler.connection.read_buf_data() => res,
        };

        let (cmd, vec) = match result {
            Some((cmd, vec)) => (cmd, vec),
            None => return Ok(()),
        };

        handler.process_query(cmd, vec).await?;
    }
    Ok(())
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        Connection { stream: stream }
    }

    pub async fn rea_buf_data(&mut self) -> Option<(Command, Vec<String>)> {
        let mut buf = ButesMut::with_capacity(1024);
        match self.stream.read_buf(&mut buf).await {
            Ok(size) => {
                if size == 0 {
                    // returning from empty buffer
                    return None;
                }
            }
            Err(err) => {
                println!("error {:?}", err);
                return None;
            }
        };
        let attrs = buffer_to_array(&mut buf);
        Some((Command::get_command(&attrs[0]), attrs))
    }
}

impl Shutdown {
    fn new(shutdown: bool, notify: broadcast:: Receiver<()>) -> Shutdown {
        Shutdown { shutdown, notify }
    }

    pub async fn listen_recv(&mut self) -> Result<(), tokio::sync::broadcast::error::RecvError> {
        self.notify.recv().await?;
        self.shutdown = true;
        Ok(())
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    impl Handler {
        pub fn new(listener: &Listener, socket: TcpStream) -> Handler {
            Handler {
                connection: Connection::new(socket),
                db: listener.db.clone(),
                shutdown: Shutdown::new(false, listener,notify_shutdown.subscribe()),
                _shutdown_complete: listener.shutdown_complete_tx.clone(),
            }
        }

        pub async fn process_query(
            &mut self,
            command: Command,
            attrs: Vec<String>,
        ) -> Result<(), std::io::Error> {

        }
    }
}
