use core::panic;
use std::collections::HashMap;
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::routing::{KBucket, KadId, RoutingEntry, RoutingTable};
use crate::rpc::{get_value, kad_packet, packet_body, request_types, response_types, SessionTable};
use rand::{Error, Rng};
use tokio::net::UdpSocket;

#[derive(Clone)]
pub struct Node {
    pub id: KadId,
    pub routing_table: Arc<Mutex<RoutingTable>>,
    pub hash_table: Arc<Mutex<HashMap<KadId, KadId>>>,
    pub session_table: Arc<Mutex<SessionTable>>,
    pub sock: Arc<UdpSocket>,
    pub addr: String,
}

impl Node {
    // create a new node
    pub async fn new(ip: String) -> Result<Node, String> {
        // generate a random id
        let mut rng = rand::thread_rng();
        let id: KadId = rng.gen();
        let s = UdpSocket::bind(&ip).await;
        if let Ok(socket) = s {
            // start the listener thread here

            return Ok(Node {
                id,
                routing_table: Arc::new(Mutex::new(RoutingTable::new(id, 20, 160))),
                hash_table: Arc::new(Mutex::new(HashMap::new())),
                session_table: Arc::new(Mutex::new(SessionTable::new())),
                sock: Arc::new(socket),
                addr: ip,
            });
        } else {
            return Err(String::from("Failed to bind socket"));
        }
    }

    // start the server
    pub fn start_listen(self) {
        // this function will start a task that listens for incoming RPCs to the node
        tokio::spawn(async move {
            loop {
                // listen on the socket
                let mut buffer = [0_u8; 1024];
                match self.sock.recv_from(&mut buffer).await {
                    Ok((_, addr)) => {
                        // deserialize and process accordingly
                        let data: Result<kad_packet, serde_json::Error> =
                            serde_json::from_slice(&buffer);
                        if let Ok(packet) = data {
                            // processes the incoming packet in a new task
                            self.clone().process_packet(packet, addr);
                        }
                    }
                    Err(_) => {
                        panic!("im lazy idk");
                    }
                }
            }
        });
    }

    // join an existing network
    pub async fn join() {
        return;
    }

    pub async fn ping(&mut self) {
        return;
    }

    pub async fn store(&mut self) {
        return;
    }

    pub async fn find_node(&mut self) {
        return;
    }

    pub async fn find_value(&mut self) {
        return;
    }

    // process an incoming packet
    pub fn process_packet(self, packet: kad_packet, addr: SocketAddr) {
        tokio::spawn(async move {
            if packet.dest != self.id {
                return;
            }

            //TODO: do something here where you update the routing table

            match packet.body {
                packet_body::REQUEST(_) => self.process_req(packet, addr).await,
                packet_body::RESPONSE(_) => self.process_rsp(packet).await,
            }
        });
    }

    // process an incoming packet
    pub async fn process_req(self, req: kad_packet, addr: SocketAddr) {
        return;
    }

    // process a response to one of our rpcs
    pub async fn process_rsp(self, rsp: kad_packet) {
        let mut table = self.session_table.lock().await;
        if let Some(_) = table.get(rsp.txid) {
            table.update(rsp.txid, rsp);
        }
    }
}
