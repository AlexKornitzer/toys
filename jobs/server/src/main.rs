// Design
//
// 1. 6000:
//  a. Pub/Sub - used for state updates/verification and heartbeats
//  b. Survey - used for leader election
// 2. 6001:
//  a. Req/Rep - used for comms to leader (i.e. mutating state changes, etc)

use nng::{Protocol, Socket};
use uuid::Uuid;

fn respondent(address: &str) -> Result<(), nng::Error> {
    let uuid = Uuid::new_v4();
    println!("{}", uuid);
    let socket = Socket::new(Protocol::Respondent0)?;
    socket.dial(&address)?;
    loop {
        if let Ok(msg) = socket.recv() {
            println!("{}", String::from_utf8_lossy(msg.as_slice()));
            socket.send(format!("{}", uuid).as_bytes()).unwrap();
        }
    }
}

fn surveyor(address: &str) -> Result<(), nng::Error> {
    let uuid = Uuid::new_v4();
    println!("{}", uuid);
    let socket = Socket::new(Protocol::Surveyor0)?;
    socket.listen(&address)?;
    let mut count = 0;
    loop {
        socket.send(format!("{}", count).as_bytes()).unwrap();
        loop {
            let resp = socket.recv();
            match resp {
                Ok(msg) => println!("{}", String::from_utf8_lossy(msg.as_slice())),
                Err(nng::Error::TimedOut) => break,
                Err(err) => return Err(err),
            }
        }
        count += 1;
    }
}

fn main() {
    let args = std::env::args();
    if args.len() < 2 {
        return;
    }
    let mut args = args.into_iter();
    args.next();
    let address = format!("tcp://{}:{}", args.next().unwrap(), args.next().unwrap());

    if let Ok(_) = respondent(&address) {
        println!("was respondent");
    } else if let Ok(_) = surveyor(&address) {
        println!("was surveyor");
    }
}
