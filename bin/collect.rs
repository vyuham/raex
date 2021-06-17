use bytes::Bytes;
use dstore::Local;
use raex::{
    rtrc::{IMAGE_HEIGHT, IMAGE_WIDTH},
    DIV,
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
    for cursor in (0..IMAGE_HEIGHT / DIV).into_iter().rev() {
        eprint!("\rCursor: {} ", cursor);
        stderr().flush().unwrap();
        if let Ok((_, pixels)) = local
            .lock()
            .await
            .get(&Bytes::from(vec![cursor as u8]))
            .await
        {
            for pixel in pixels.chunks(3) {
                println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
            }
        } else {
            eprintln!("{}", cursor);
        }
    }

    eprint!("\rImage Generated!");
    stderr().flush().unwrap();

    Ok(())
}
