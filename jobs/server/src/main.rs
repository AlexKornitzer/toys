#[macro_use]
extern crate anyhow;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use quorum::{Floor, Lobby};
use structopt::StructOpt;
use tracing::info;
use tracing_subscriber;

#[derive(StructOpt)]
pub struct Opts {
    #[structopt(default_value = "srvr0")]
    name: String,
    #[structopt(default_value = "6000")]
    port: u16,
    peers: Vec<String>,
}

pub enum Request {
    Pop,
    Push,
}
pub struct Manager;
#[derive(Default)]
pub struct Stack {
    inner: Vec<String>,
}
impl Stack {
    pub fn pop(&self) {}
    pub fn push(&self) {}
}
pub struct State {
    pub stack: Stack,
}

fn main() {
    tracing_subscriber::fmt::init();
    let opts = Opts::from_args();
    let state = Arc::new(State {
        stack: Stack::default(),
    });
    let s = state.clone();
    let manifest = move |v| {
        let req = Request::Pop;
        match req {
            Request::Pop => s.stack.pop(),
            Request::Push => s.stack.push(),
        }
    };
    let forum = Lobby::register("127.0.0.1".to_owned())
        .peers(opts.peers)
        .port(opts.port)
        .enter(manifest)
        .unwrap();
    let manager = Arc::new(Manager);
    let m = manager.clone();
    let s = state.clone();
    let cmd = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            // receive some message...
            let req = Request::Pop;
            match forum.floor() {
                Some(Floor::Chair(chair)) => {
                    info!("chair");
                    match req {
                        Request::Pop => {
                            s.stack.pop();
                            chair.decree("bloop".to_owned()).unwrap();
                        }
                        Request::Push => {
                            s.stack.push();
                            chair.decree("bloop".to_owned()).unwrap();
                        }
                    }
                }
                Some(Floor::Member(member)) => {
                    info!("member");
                    member.propose("bloop".to_owned()).unwrap();
                    // ACTION...
                }
                None => {
                    info!("hung");
                }
            }
        }
    });
    cmd.join().unwrap();
}
