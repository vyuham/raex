use bytes::Bytes;
use dstore::Local;
use raex::rtrc::{IMAGE_HEIGHT, IMAGE_WIDTH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local = Local::new("[::1]:50051", "[::1]:50052").await?;

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
