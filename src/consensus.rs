   use std::{collections::Hashmap, sync::Arc};
   use::tokio::sync::{mpsc, Mutex};
   use raft::{
    Config,
    raw_node::RawNode,
    storage::{MemStorage, Storage},
    raftpb::{Entry, EntryType, Message},
    SnapshotStatus,
    };
   use slog::{info, o, Drain, Logger};

   #[derive(Debug, Clone)]
   pub enum Command {
    Set {key:String , value:String}
    Delete {key:String}
   }

   impl Command {
        pub fn serialize(&self) -> Vec<u8> {
            match self {
                command::Set { key, value } => format!("SET {} {}", key, value).into_bytes(),
                Command::Delete {key} => format!("DEL {}", Key).into_bytes(),
                
            }
        } 

        pub fn deserialize(data: &[u8]) -> Option<Self> {
            let s = String::from_utf8_lossy(data);
            
        }
   }
