use clap::Parser;

#[derive(Parser, Debug)]
pub struct args {
    
    pub id : i64,

    pub peers :String,

    pub http_addr : String,

    pub raft_addr : String,

}