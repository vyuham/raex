use bytes::Bytes;
use dstore::{Local, Queue};
use raex::rtrc::RayTracer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut queue, local, tracer) = (
        Queue::connect("[::1]:50051").await?,
        Local::new("[::1]:50051", "[::1]:50052").await?,
        RayTracer::default(),
    );
    loop {
        if let Ok(popped) = queue.pop_front(Bytes::from("tasks")).await {
            let popped = popped.to_vec();
            let (i, j) = ((popped[0] as u16) << 8 | popped[1] as u16, popped[2] as u16);
            eprint!("[{}, {}] :", i, j);
            let pixel = tracer.render(i, j);
            let _ = local
                .lock()
                .await
                .insert(Bytes::from(popped), Bytes::from(pixel))
                .await;
        } else {
            eprintln!("There is no more things to get from the queue");
            break;
        }
    }

    Ok(())
}
