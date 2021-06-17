use bytes::Bytes;
use dstore::{Local, Queue};
use raex::{
    rtrc::{RayTracer, IMAGE_WIDTH},
    DIV,
};
use rayon::prelude::*;
use std::{
    env,
    io::{stderr, Write},
    sync::Arc,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (global_addr, local_addr) = (&args[1], &args[2]);

    let (tracer, local) = (
        Arc::new(RayTracer::default()),
        Local::new(global_addr, local_addr).await?,
    );

    loop {
        if let Ok(popped) = Queue::connect(global_addr)
            .await?
            .pop_front(Bytes::from("tasks"))
            .await
        {
            let popped = popped.to_vec();
            let cursor = popped[0] as i32 * DIV;
            let scanlines = (cursor..cursor + DIV)
                .into_par_iter()
                .rev()
                .flat_map(|j| {
                    eprint!("\rScanline: {}", j);
                    let mut scanline = vec![];
                    for i in 0..IMAGE_WIDTH {
                        stderr().flush().unwrap();
                        scanline.append(&mut tracer.render(i as u16, j as u16));
                    }
                    scanline
                })
                .collect::<Vec<u8>>();

            let _ = local
                .lock()
                .await
                .insert(Bytes::from(popped), Bytes::from(scanlines))
                .await;
        } else {
            eprintln!("\rThere is no more tasks to get from the queue");
            stderr().flush().unwrap();
            break;
        }
    }

    Ok(())
}
