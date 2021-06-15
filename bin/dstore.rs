use dstore::Global;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];

    println!("Dstore server listening on {}", addr);
    Global::start_server(addr).await
}
