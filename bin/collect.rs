use bytes::Bytes;
use dstore::Local;
use raex::rtrc::{IMAGE_HEIGHT, IMAGE_WIDTH};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (global_addr, local_addr) = (&args[1], &args[2]);

    let local = Local::new(global_addr, local_addr).await?;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for i in (0..IMAGE_WIDTH).into_iter().rev() {
        for j in 0..IMAGE_HEIGHT {
            let (_, pixel) = local
                .lock()
                .await
                .get(&Bytes::from(vec![(i >> 8) as u8, i as u8, j as u8]))
                .await?;
            println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
        }
    }

    Ok(())
}
