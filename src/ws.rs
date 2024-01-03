use crate::{Client, Clients};
use warp::ws::WebSocket;

pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("Establishing connection... {:?}", ws);


}
