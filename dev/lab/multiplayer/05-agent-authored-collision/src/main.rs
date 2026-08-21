use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use agent_authored_collision_lab::{CollisionLab, app};

#[tokio::main]
async fn main() {
    let requested = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:0".to_owned());
    let requested: SocketAddr = requested
        .parse()
        .expect("usage: agent-authored-collision-lab [127.0.0.1:PORT]");
    assert!(
        requested.ip().is_loopback(),
        "the lab only binds to loopback"
    );
    let listener = tokio::net::TcpListener::bind(requested)
        .await
        .expect("the lab listener must bind");
    let address = listener.local_addr().expect("the lab address must exist");
    let router = app(Arc::new(Mutex::new(CollisionLab::fixture())), address)
        .expect("the loopback lab app must build");
    println!("{{\"event\":\"server_ready\",\"address\":\"{address}\"}}");
    axum::serve(listener, router)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .expect("the lab server must run");
}
