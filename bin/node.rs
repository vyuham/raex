use bytes::Bytes;
use dstore::{Local, Queue};
use raex::rtrc::RayTracer;
use std::{env, sync::Arc};

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
                let (i, j) = ((popped[0] as u16) << 8 | popped[1] as u16, popped[2] as u16);
                eprintln!("[{}, {}]", i, j);
                let pixel = tracer_ref.render(i, j);
                let _ = local_ref
                    .lock()
                    .await
                    .insert(Bytes::from(popped), Bytes::from(pixel))
                    .await;
            });
        } else {
            eprintln!("There is no more things to get from the queue");
            break;
        }
    }

    Ok(())
}
