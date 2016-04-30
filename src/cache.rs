use std::collections::HashMap;

use errs::Error;
use matches::{MatchResults};
use bucket::{BucketBuilder, Bucket, ReadHandle, WriteHandle, BucketStats};

#[derive(Debug)]
pub struct CacheStats {
    buckets: HashMap<String, BucketStats>,
    columns: usize,
    inserts: usize,
    deletes: usize,
    rows: usize,
}

pub struct Cache<'c> {
    buckets: HashMap<&'c str, Bucket<'c>>,
}

impl<'c> Cache<'c> {
    pub fn new() -> Self {
        Cache {
            buckets: HashMap::new(),
        }
    }

    pub fn stats(&self) -> CacheStats {
        let mut s = CacheStats {
            buckets: HashMap::new(),
            columns: 0,
            inserts: 0,
            deletes: 0,
            rows: 0
        };
        for (name, bucket) in self.buckets.iter() {
            let bs = bucket.stats();
            s.columns += bs.columns;
            s.inserts += bs.inserts;
            s.deletes += bs.deletes;
            s.rows += bs.inserts - bs.deletes;
            s.buckets.insert(name.to_string(), bs);
        }
        s
    }

    pub fn new_bucket(&mut self, bb: BucketBuilder<'c>) -> Result<(), Error> {
        let name = bb.name;
        let rb = Bucket::new(bb.columns);
        match rb {
            Ok(b) => {
                self.buckets.insert(name, b);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn bucket<F>(&self, bucket_name: &str, closure: F) where F: FnOnce(Option<ReadHandle>) {
        match self.buckets.get(bucket_name) {
            Some(b) => closure(Some(ReadHandle::new(b))),
            _ => closure(None),
        }
    }

    pub fn bucket_mut<F>(&mut self, bucket_name: &str, closure: F) where F: FnOnce(Option<WriteHandle>) {
        match self.buckets.get_mut(bucket_name) {
            Some(b) => {
                b.write().unwrap();
                closure(Some(WriteHandle::new(b)))
            },
            _ => closure(None),
        }
    }
}
