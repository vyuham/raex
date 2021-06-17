use bytes::Bytes;
use dstore::Queue;
use raex::{rtrc::IMAGE_HEIGHT, DIV};
use std::{
    env,
    io::{stderr, Write},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut queue = Queue::connect(&args[1]).await?;
    for cursor in (0..IMAGE_HEIGHT / DIV).into_iter().rev() {
        eprint!("\rCursor: {} ", cursor);
        stderr().flush().unwrap();
        let _ = queue
            .push_back(Bytes::from("tasks"), Bytes::from(vec![cursor as u8]))
            .await;
    }

    Ok(())
}
