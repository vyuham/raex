use bytes::Bytes;
use dstore::{Local, Queue};
use raex::rtrc::RayTracer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tracer, local) = (
        Arc::new(RayTracer::default()),
        Local::new("[::1]:50051", "[::1]:50052").await?,
    );
    loop {
        if let Ok(popped) = Queue::connect("[::1]:50051")
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
