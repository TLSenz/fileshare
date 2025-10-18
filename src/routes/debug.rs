use std::net::SocketAddr;
use axum::extract::ConnectInfo;

pub async fn debug_ip(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("{}", addr.ip())
}
