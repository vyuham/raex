use crate::{
    raft_proto::{
        raft_client::RaftClient,
        raft_server::{Raft, RaftServer},
        Byte, Null,
    },
    RaExConfig,
};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    sync::Arc,
};
use tokio::sync::Mutex;
use tonic::{
    transport::{Channel, Server},
    Request, Response, Status,
};

struct State<T> {
    pub schedule: Arc<Mutex<VecDeque<T>>>,
    pub executing: Arc<Mutex<HashMap<i8, T>>>,
    pub log: Arc<Mutex<Vec<(u64, i8, T)>>>,
}

pub struct Consensus<T> {
    state: State<T>,
    clients: Vec<RaftClient<Channel>>,
}

impl<T: Sync + Send + 'static> Consensus<T> {
    pub async fn start(cfg: Arc<RaExConfig>) -> Result<Self, Box<dyn Error>> {
        let (schedule, executing, log) = (
            Arc::new(Mutex::new(VecDeque::new())),
            Arc::new(Mutex::new(HashMap::new())),
            Arc::new(Mutex::new(vec![])),
        );

        let mut nodes = cfg.nodes.clone();
        nodes.retain(|x| *x != cfg.local_addr);

        let mut clients = vec![];

        for node in nodes {
            clients.push(RaftClient::connect(format!("http://{  }", node)).await?);
        }

        // State that is handed over the the server stub on this node
        let state = State {
            schedule: schedule.clone(),
            executing: executing.clone(),
            log: log.clone(),
        };

        // Server runs on a background thread and handles calls to the node
        tokio::spawn(async move {
            Server::builder()
                .add_service(RaftServer::new(state))
                .serve(cfg.local_addr.parse().unwrap())
                .await;
        });

        Ok(Self {
            state: State {
                schedule,
                executing,
                log,
            },
            clients,
        })
    }
}

#[tonic::async_trait]
impl<T: Sync + Send + 'static> Raft for State<T> {
    async fn join(&self, args: Request<Byte>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null {}))
    }
}
