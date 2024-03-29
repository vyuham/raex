use bytes::Bytes;
use dstore::Local;
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
    let (global_addr, local_addr) = (&args[1], &args[2]);

    let local = Local::new(global_addr, local_addr).await?;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).into_iter().rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let (_, pixel) = local
                .lock()
                .await
                .get(&Bytes::from(coord_vec(i as u16, j as u16)))
                .await?;
            println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
        }
    }

    eprint!("\rImage Generated!");
    stderr().flush().unwrap();

    Ok(())
}
