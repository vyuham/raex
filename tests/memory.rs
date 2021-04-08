macro_rules! bytes {
    ($e: expr) => {
        bytes::Bytes::from($e)
    };
}

#[tokio::test(threaded_scheduler)]
async fn connect_memory() {
    tokio::spawn(async {
        dstore::Global::start_server("127.0.0.1:50051".parse().unwrap())
            .await
            .unwrap()
    });

    tokio::spawn(async {
        let mem = dstore::Local::new("127.0.0.1:50051".to_string(), "127.0.0.1:50052".to_string())
            .await
            .unwrap();

        if let Err(_) = mem
            .lock()
            .await
            .insert(bytes!("Hello"), bytes!("World"))
            .await
        {
            panic!()
        }

        assert_eq!(
            mem.lock().await.get(&bytes!("Hello")).await.unwrap(),
            "World"
        );
    });
}
