use clap::Parser;
use dirs;
use futures::channel::{mpsc as futures_mpsc, oneshot};
use futures::prelude::*;
use futures::StreamExt;
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{identity, ping, Multiaddr, PeerId};

use log::{debug, info};

use std::{
    error::Error,
    fs,
    io::{Read, Write},
    os::unix::fs::OpenOptionsExt,
    sync::{Arc, Mutex},
};
use tokio::io;

// use pyrsia_blockchain_network::blockchain::Blockchain;
pub mod args;
use args::parser::BlockchainNodeArgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let args = BlockchainNodeArgs::parse();

    let key_path = get_keyfile_name(args.clone());

    // If the key file exists, load the key pair. Otherwise, create a random keypair and save to the keypair file
    let id_keys = create_ed25519_keypair(key_path);
    let ed25519_pair = identity::Keypair::Ed25519(id_keys.clone());
    let _peer_id = PeerId::from(ed25519_pair.public());

    println!("Getting network up! PeerID{:?}", _peer_id);

    let transport = libp2p::development_transport(ed25519_pair).await?;

    // Create a ping network behaviour.
    //
    // For illustrative purposes, the ping protocol is configured to
    // keep the connection alive, so a continuous sequence of pings
    // can be observed.
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, _peer_id);

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = args.addr {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }
}

pub fn write_keypair(path: &String, data: &[u8; 64]) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o600)
        .open(path)
        .expect("cannot open file");

    file.write_all(data).expect("write failed");
}

pub fn read_keypair(path: &String) -> Result<[u8; 64], Box<dyn Error>> {
    let mut file = std::fs::File::open(path)?;
    let mut buf = [0u8; 64];
    let n = file.read(&mut buf)?;
    if n == 64 {
        Ok(buf)
    } else {
        Err(Box::new(io::Error::from(io::ErrorKind::InvalidData)))
    }
}

pub fn get_keyfile_name(args: BlockchainNodeArgs) -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(args.key_filename);
    let filepath = path.into_os_string().into_string().unwrap();
    filepath
}

pub fn create_ed25519_keypair(filename: String) -> libp2p::identity::ed25519::Keypair {
    match read_keypair(&filename) {
        Ok(v) => {
            let data: &mut [u8] = &mut v.clone();
            debug!("Load Keypair from {:?}", filename);
            libp2p::identity::ed25519::Keypair::decode(data).unwrap()
        }
        Err(_) => {
            let id_keys = identity::ed25519::Keypair::generate();

            let data = id_keys.encode();
            debug!("Create Keypair");
            write_keypair(&filename, &data);
            id_keys
        }
    }
}
