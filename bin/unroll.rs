use bytes::Bytes;
use dstore::Queue;
use raex::{
    coord_vec,
    rtrc::{IMAGE_HEIGHT, IMAGE_WIDTH},
};
use std::{
    env,
    io::{stderr, Write},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut queue = Queue::connect(&args[1]).await?;
    for j in (0..IMAGE_HEIGHT).into_iter().rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let _ = queue
                .push_back(Bytes::from("tasks"), Bytes::from(coord_vec(i as u16, j as u16)))
                .await;
        }
    }

    Ok(())
}
