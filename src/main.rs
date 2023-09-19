use std::{collections::VecDeque, time::Instant};

struct KBucketEntry {
    last_seen: Instant,
    ip: String,
    port: String,
    id: String,
}

impl KBucketEntry {
    fn new(ip: String, port: String, id: String) -> KBucketEntry {
        KBucketEntry {
            last_seen: Instant::now(),
            ip,
            port,
            id,
        }
    }
}

struct KBucket {
    size: usize,
    list: VecDeque<KBucketEntry>,
}

impl KBucket {
    // creates a new KBucket
    fn new(size: usize) -> KBucket {
        KBucket {
            size: size,
            list: VecDeque::new(),
        }
    }

    // add a record to the kbucket
    fn put(&mut self, ip: String, port: String, id: String) {
        // check if the item already exists in the k-bucket, if so update it
        if let Some(_) = self.get(id.clone()) {
            self.remove_id(id.clone());
        }

        // check if the list is full
        if self.list.len() == self.size {
            self.remove_oldest();
        }

        // create a new entry and add it to the list
        let entry = KBucketEntry::new(ip, port, id);
        self.list.push_back(entry);
    }

    // get a record from the bucket by idf
    fn get(&mut self, id: String) -> Option<&KBucketEntry> {
        for i in self.list.iter() {
            if i.id == id {
                return Some(i);
            }
        }
        None
    }

    // remove oldest item from the kbucket
    fn remove_oldest(&mut self) {
        let mut least_recent = Instant::now();
        let mut least_recent_i = 0;
        // iterate over the list and find the maximum time
        for (j, k) in self.list.iter().enumerate() {
            if j == 0 {
                least_recent = k.last_seen;
            } else {
                if least_recent > k.last_seen {
                    least_recent = k.last_seen;
                    least_recent_i = j;
                }
            }
        }
        self.list.remove(least_recent_i);
    }

    // remove item in k-bucket at a specific index
    fn remove_id(&mut self, id: String) {
        let mut i: Option<usize> = None;
        // iterate over the list and find the maximum time
        for (j, k) in self.list.iter().enumerate() {
            if k.id == id {
                i = Some(j);
                break;
            }
        }
        if let Some(index) = i {
            self.list.remove(index);
        }
    }

    fn get_list(&self) -> &VecDeque<KBucketEntry> {
        &self.list
    }
}

fn main() {
    println!("Hello, world!");
}
