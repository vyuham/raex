use bytes::Bytes;
use dstore::Queue;
use raex::rtrc::{IMAGE_HEIGHT, IMAGE_WIDTH};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut queue = Queue::connect(&args[1]).await?;
    for i in (0..IMAGE_WIDTH).into_iter().rev() {
        for j in 0..IMAGE_HEIGHT {
            let _ = queue
                .push_back(
                    Bytes::from("tasks"),
                    Bytes::from(vec![(i >> 8) as u8, i as u8, j as u8]),
                )
                .await;
        }
    }

    Ok(())
}
