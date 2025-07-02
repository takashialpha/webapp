use axum::{routing::get, Router};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = create_reuseport_listener(addr).unwrap();

    let app = Router::new().route("/", get(root));

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn create_reuseport_listener(addr: SocketAddr) -> std::io::Result<TcpListener> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_port(true)?;
    socket.set_reuse_address(true)?;
    socket.bind(&addr.into())?;
    socket.listen(1024)?;
    
    let std_listener: std::net::TcpListener = socket.into();

    std_listener.set_nonblocking(true)?;

    TcpListener::from_std(std_listener)
}

