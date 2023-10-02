use std::{
    collections::HashMap,
    time::{Duration, Instant},
    usize,
};

// k bucket entries
pub type NodeId = [u8; 20];
pub struct RoutingEntry {
    id: NodeId,
    ip: String,
    port: String,
    last_seen: Instant,
}

// implement the new method for routing entries
impl RoutingEntry {
    pub fn new(id: NodeId, ip: String, port: String) -> RoutingEntry {
        RoutingEntry {
            id,
            ip,
            port,
            last_seen: Instant::now(),
        }
    }
}

// the k-bucket
const KBUCKET_SIZE: usize = 20;
pub struct KBucket {
    pub size: usize,
    pub entries: HashMap<NodeId, RoutingEntry>,
}

// implement the k-bucket
impl KBucket {
    pub fn new(size: usize) -> KBucket {
        KBucket {
            size,
            entries: HashMap::new(),
        }
    }

    // check whether the bucket is at max capacity
    pub fn is_full(&self) -> bool {
        return self.size == self.entries.len();
    }

    // get an item from the bucket by id
    pub fn get(&self, id: NodeId) -> Option<&RoutingEntry> {
        self.entries.get(&id)
    }

    // remove an item from the bucket and return it
    pub fn remove(&mut self, id: NodeId) -> Option<RoutingEntry> {
        self.entries.remove(&id)
    }

    // fn get oldest
    pub fn get_oldest(&self, id: NodeId) -> Option<&RoutingEntry> {
        self.entries.values().max_by_key(|&v| v.last_seen)
    }

    // update the time seen of some item
    pub fn touch(&mut self, id: NodeId) -> Option<&RoutingEntry> {
        match self.entries.get_mut(&id) {
            Some(entry) => {
                entry.last_seen = Instant::now();
                Some(entry)
            }
            None => {
                return None;
            }
        }
    }

    // try to add an item
    pub fn try_add(&mut self, id: NodeId, ip: String, port: String) -> Option<&RoutingEntry> {
        let entry = RoutingEntry::new(id, ip, port);
        if self.is_full() {
            return None;
        } else {
            self.entries.insert(id, entry);
            Some(self.get(id)?)
        }
    }
}

// struct representing the routing table
pub struct RoutingTable {
    pub local_id: NodeId,
    buckets: Vec<KBucket>,
}

impl RoutingTable {
    // initialize a new routing table
    pub fn new(id: NodeId, bucket_size: usize, id_bits: usize) -> RoutingTable {
        let mut buckets: Vec<KBucket> = Vec::with_capacity(id_bits);
        for i in 0..id_bits {
            buckets[i] = KBucket::new(bucket_size);
        }
        RoutingTable {
            local_id: id,
            buckets: buckets,
        }
    }

    // get the xor distance from the node's id to another id
    pub fn distance_to(&self, id: NodeId) -> usize {
        // xor the butes and calculate the leading zeros
        let mut dist: usize = 0;
        for i in 0..20 {
            let xor_bytes = self.local_id[i] ^ id[i];
            dist += xor_bytes.leading_zeros() as usize;
            if xor_bytes != 0 {
                break;
            }
        }
        // notice that we calculated the number of leading zeros, not the distance
        160 - dist
    }

    // get the bucket of nodes at the xor distance to an id
    pub fn get_bucket(&self, id: NodeId) -> &KBucket {
        let bucket = &self.buckets[self.distance_to(id)];
        bucket
    }
}
