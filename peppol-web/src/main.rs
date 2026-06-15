// Peppol Web Frontend — Order Capture
//
// Simple Axum server serving the UBL Order form on :3001.
// Submits validated orders to peppol-api on :3000.

use axum::response::Html;
use axum::routing::get;
use std::net::SocketAddr;

async fn order_form() -> Html<&'static str> {
    Html(include_str!("order_form.html"))
}

fn main() {
    tracing_subscriber::fmt::init();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let app = axum::Router::new().route("/", get(order_form));

        let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
        tracing::info!("Peppol Web UI listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}
