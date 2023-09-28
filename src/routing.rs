mod routing {
    use crate::k_bucket::KBucket;
    use crate::k_bucket::KBucketEntry;
    use crate::k_bucket::NodeId;
    use crate::k_bucket::KBUCKET_SIZE;

    struct RoutingTable {
        id: NodeId,
        buckets: Vec<KBucket>,
    }

    impl RoutingTable {
        // initializes a routing table
        pub fn new(id: NodeId) -> RoutingTable {
            let mut b: Vec<KBucket> = Vec::with_capacity(160);
            for i in 0..160 {
                b[i] = KBucket::new(KBUCKET_SIZE);
            }
            let buckets = b;
            RoutingTable { id, buckets }
        }

        // takes the xor of the callign node's id and another id
        pub fn xor_id(&self, id: NodeId) -> NodeId {
            let mut x: NodeId = [0; 20];
            for i in 0..20 {
                x[i] = self.id[i] ^ id[i];
            }
            x
        }

        // determines the xor distance between the calling node's id and another id
        pub fn distance(&self, id: NodeId) -> usize {
            let mut dist: u32 = 0;
            let id = self.xor_id(id);

            for i in id.iter() {
                let lz = i.leading_zeros();
                dist += lz;
                if lz != 8 {
                    break;
                }
            }
            dist as usize
        }

        // update the routing table
        pub fn update(&mut self, ip: String, port: String, id: NodeId) {
            // find the proper k bucket for the new entry
            let dist = self.distance(id);
            self.buckets[dist].try_add(ip, port, id);
        }

        // returns the k-bucket that has the closest xor distance to the given id
        pub fn get_closest(&self, id: NodeId) -> &KBucket {
            let dist = self.distance(id);
            &self.buckets[dist]
        }
    }
}
