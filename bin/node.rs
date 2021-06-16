use bytes::Bytes;
use dstore::{Local, Queue};
use raex::{rtrc::RayTracer, vec_coord};
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
            let (local_ref, tracer_ref) = (local.clone(), tracer.clone());
            tokio::spawn(async move {
                let popped = popped.to_vec();
                let (i, j) = vec_coord(&popped);
                eprint!("\r[{}, {}]", i, j);
                stderr().flush().unwrap();
                let pixel = tracer_ref.render(i, j);
                let _ = local_ref
                    .lock()
                    .await
                    .insert(Bytes::from(popped), Bytes::from(pixel))
                    .await;
            });
        } else {
            eprintln!("\rThere is no more tasks to get from the queue");
            stderr().flush().unwrap();
            break;
        }
    }

    Ok(())
}
