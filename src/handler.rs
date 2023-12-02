pub struct Handler {
    pub connection: Connection,
    pub db: Db,
    pub shutdown: Shutdown,
    //  When the handler object is dropped, this sends a message to the receiver
    _shutdown_complete: mpsc::Sender(),
}

pub struct Connection {
    pub stream: TcpStream,
}

pub struct Shutdown {
    shutdown: bool,
    notify: broadcast::Receiver<()>,
}
