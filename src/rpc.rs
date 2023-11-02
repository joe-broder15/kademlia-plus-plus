use crate::routing::{KadId, RoutingEntry};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use serde::{Serialize, Deserialize};

// types of requests
#[derive(Debug, Serialize, Deserialize)]
pub enum request_types {
    PING,
    STORE(KadId, KadId),
    GET_NODE(KadId),
    GET_VALUE(KadId),
}
// option for GET_VALUE as it can return either a list of entries or a value
#[derive(Debug, Serialize, Deserialize)]
pub enum get_value {
    VALUE(KadId),
    NODES(Vec<RoutingEntry>),
}

// types of requests
#[derive(Debug, Serialize, Deserialize)]
pub enum response_types {
    PING(bool),
    STORE(bool),
    GET_NODE(Vec<RoutingEntry>),
    GET_VALUE(get_value),
}

// body types
#[derive(Debug, Serialize, Deserialize)]
pub enum packet_body {
    REQUEST(request_types),
    RESPONSE(response_types),
}

// general header for all rpc transactions
#[derive(Debug, Serialize, Deserialize)]
pub struct kad_packet {
    pub src: KadId,
    pub dest: KadId,
    pub txid: KadId,
    pub body: packet_body,
}

pub struct SessionEntry {
    txid: KadId,
    req: kad_packet,
    rsp: Option<kad_packet>,
    created: Instant,
}

// we are going to implement a future method for the SessionEntry so that we can wait for a response
impl Future for SessionEntry {
    type Output = bool;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool> {
        // returns true if there is a response for the packet
        if let Some(rsp) = &self.rsp {
            Poll::Ready(true)
        // returns false if the packet hasn't seen a response in 10 seconds
        } else if self.created.elapsed().as_secs() > 10 {
            Poll::Ready(false)
        // otherwise keep sleeping
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub struct SessionTable {
    entries: HashMap<KadId, SessionEntry>,
}

impl SessionTable {
    pub fn new() -> SessionTable {
        SessionTable {
            entries: HashMap::new(),
        }
    }

    // get a session entry from the table
    pub fn get(&self, txid: KadId) -> Option<&SessionEntry> {
        self.entries.get(&txid)
    }

    // add a packet to the session table
    pub fn add(&mut self, txid: KadId, packet: kad_packet) {
        let entry = SessionEntry {
            txid,
            req: packet,
            rsp: None,
            created: Instant::now(),
        };
        self.entries.insert(txid, entry);
    }

    // fulfill the transaction and add a response
    pub fn update(&mut self, txid: KadId, rsp: kad_packet) {
        if let Some(s) = self.entries.get_mut(&txid) {
            s.rsp = Some(rsp);
        }
    }

    // remove a session from the table
    pub fn remove(&mut self, txid: KadId) {
        self.entries.remove(&txid);
    }
}
