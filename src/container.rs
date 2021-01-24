use crate::{config::RaexConfig, executor::Exec, memory::Memory, raft::Raft};

pub struct Container {
    exec: Exec,
    mem: Memory,
    config: RaexConfig,
    raft: Raft,
}
