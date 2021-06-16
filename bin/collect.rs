use bytes::Bytes;
use dstore::Local;
use raex::{
    rtrc::{IMAGE_HEIGHT, IMAGE_WIDTH},
    to_tuple,
};
use std::{
    env,
    io::{stderr, Write},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (global_addr, local_addr) = (&args[1], &args[2]);

    let local = Local::new(global_addr, local_addr).await?;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).into_iter().rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        let (_, pixels) = local
            .lock()
            .await
            .get(&Bytes::from(to_tuple(j as u16)))
            .await?;
        for pixel in pixels.chunks(3) {
            println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
        }
    }

    eprint!("\rImage Generated!");
    stderr().flush().unwrap();

    Ok(())
}
