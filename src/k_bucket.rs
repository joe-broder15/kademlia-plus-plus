// this encapsulates the k bucket
use std::{collections::HashMap, time::Instant};

pub const KBUCKET_SIZE: usize = 20;

// convenient alias for 160 byte hashes
pub type NodeId = [u8; 20];

// entries of the K-bucket
pub struct KBucketEntry {
    last_seen: Instant,
    ip: String,
    port: String,
    id: NodeId,
}

impl KBucketEntry {
    fn new(ip: String, port: String, id: NodeId) -> KBucketEntry {
        Self {
            last_seen: Instant::now(),
            ip,
            port,
            id,
        }
    }
}

// the k-bucket itself
pub struct KBucket {
    size: usize,
    pub entries: HashMap<NodeId, KBucketEntry>,
}

impl KBucket {
    // creates a new KBucket
    pub fn new(size: usize) -> KBucket {
        KBucket {
            size: size,
            entries: HashMap::new(),
        }
    }

    // try to add a new item to the bucket
    pub fn try_add(&mut self, ip: String, port: String, id: NodeId) {
        // create the new entry
        let item = KBucketEntry::new(ip, port, id);

        // if an item with an identical id already exists in the bucket, update (remove) it
        if let Some(_) = self.entries.get(&item.id) {
            self.entries.remove(&item.id);
            self.entries.insert(item.id, item);

        // if there is space, add the new item
        } else if self.entries.len() < self.size {
            self.entries.insert(item.id, item);

        // otherwise we remove the oldest item and then add the new item
        } else {
            let oldest = self
                .entries
                .iter()
                .min_by_key(|e| e.1.last_seen)
                .map(|(id, _)| *id);

            if let Some(oldest_id) = oldest {
                self.entries.remove(&oldest_id);
                self.entries.insert(item.id, item);
            }
        }
    }
}

pub mod test {
    use super::*;
    use sha1::{Digest, Sha1};

    #[test]
    fn test_kbucket() {
        let mut buc = KBucket::new(KBUCKET_SIZE);

        // add 100 things to the k-bucket
        for _ in 0..2 {
            for i in 0..100_u8 {
                // create a hasher and hash the index
                let mut hasher = Sha1::new();
                hasher.update([i]);

                // get the hash
                let mut id: NodeId = [0; 20];
                id.copy_from_slice(hasher.finalize().as_slice());

                // add to the bucket
                buc.try_add(String::from(i.to_string()), String::from(i.to_string()), id);
            }
        }

        assert_eq!(buc.entries.len(), KBUCKET_SIZE);

        // check if the first 80 items no longer exist in the bucket
        for i in 0..100_u8 {
            // create a hasher and hash the index
            let mut hasher = Sha1::new();
            hasher.update([i]);

            // get the hash
            let mut id: NodeId = [0; 20];
            id.copy_from_slice(hasher.finalize().as_slice());

            if i < 80 {
                if let Some(_) = buc.entries.get(&id) {
                    panic!("k-bucket did not evict oldest member");
                }
            }
        }
    }
}
