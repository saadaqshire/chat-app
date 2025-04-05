// we will need to use ws from main and the result type in main
use crate::{ws,Result};
use warp::Reply; // we will need to occupy warp's library to imitate a reply 

// create an asynchronous function that calls itself to return it's result  
// that implements reply. it will convert into a response. 
pub async fn ws_handler(ws: warp::ws::Ws) -> Result<impl Reply> {
	println!("ws_handler"); // calls function to return result
		
	Ok(ws.on_upgrade(move |socket| {
	
	ws::client_connection(socket)
}))
}
