use warp::{Filter, Rejection};
mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
	println!("Configuring websocket route");
	let chat = warp::path("chat") // define the path as chat
	// the ws() filter will prepare websocket handshake
	.and(warp::ws())
	.and_then(handler::ws_handler);
	println!("Starting Server...");
	warp::serve(chat).run(([127,0,0,1], 8081)).await;
	
}
