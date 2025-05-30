use crate::args::Args;
use raft::{prelude::{Entry, EntryType, Message}, storage::MemStorage, Config, RawNode};
use slog::{Drain, o};
use std::{
    collections::HashMap,
    error::Error,
    sync::mpsc::{RecvTimeoutError, channel},
    time::Duration,
};
use tokio::time::Instant;

const RAFT_TICK_INTERVAL: Duration = Duration::from_millis(100);

pub async fn init_consensus(_args: &Args) -> Result<RawNode<MemStorage>, Box<dyn  Error>> {
    let config = Config {
        id: 1, 
        ..Default::default()
    };

    let logger = slog::Logger::root(slog_stdlog::Stdlog.fuse(), o!());
    let storage = MemStorage::new_with_conf_state((vec![1], vec![]));
    let node = RawNode::new(&config, storage, &logger)?;

    Ok(node)
}

enum Msg {
    Propose {
        id: u8,
        callback: Box<dyn Fn() + Send>,
    },
    #[expect(dead_code)]
    Raft(Message),
}

pub async fn run_consensus(node: &mut RawNode<MemStorage>) {
    run_consensus_receiver_loop(node).await;
    run_consensus_sender_loop(node).await;
}

async fn run_consensus_sender_loop(node: &mut RawNode<MemStorage>) {
    let (tx, rx) = channel();
    let mut remaining_timeout = RAFT_TICK_INTERVAL;

    let _ = tx.send(Msg::Propose {
        id:1,
        callback: Box::new(|| ()),
    });

    let mut cbs = HashMap::new();
    loop {
        let now = Instant::now();

        match rx.recv_timeout(remaining_timeout) {
            Ok(Msg::Propose {id, callback }) => {
                cbs.insert(id, callback);
                let result = node.propose(vec![], vec![id]);
                println!("Propose result: {:?}", result);
            }
            Ok(Msg::Raft(m)) => {
                let result = node.step(m);
                println!("Raft result: {:?}", result);
            }
            Err(RecvTimeoutError::Timeout) => (),
            Err(RecvTimeoutError::Disconnected) => unimplemented!(),
        }

        let elapsed = now.elapsed();
        if elapsed >= remaining_timeout {
            remaining_timeout = RAFT_TICK_INTERVAL;
            node.tick();
        } else {
            remaining_timeout -= elapsed; 
        }
        break;
    }
}