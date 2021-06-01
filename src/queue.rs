use dstore::Global;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051";
    println!("Dstore server listening on {}", addr);
    Global::start_server(addr).await
}
