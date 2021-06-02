use bytes::Bytes;
use dstore::Queue;
use raex::rtrc::{IMAGE_HEIGHT, IMAGE_WIDTH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut queue = Queue::connect("[::1]:50051").await?;
    for i in (0..IMAGE_WIDTH).into_iter().rev() {
        for j in 0..IMAGE_HEIGHT {
            eprint!("[{}, {}] ", i, j);
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
