use std::collections::HashMap;

use errs::Error;
use column::ColumnRef;
use value::Value;
use matches::{Match, MatchResults};
use pattern::Pattern;
use bucket::{BucketBuilder, Bucket, ReadHandle, WriteHandle};

pub struct CacheStats {
    buckets: usize,
    columns: usize,
    rows: usize,
    inserts: usize,
    deletes: usize,
}

pub struct Cache<'c> {
    buckets: HashMap<&'c str, Bucket<'c>>,
    stats: CacheStats,
}

impl<'c> Cache<'c> {
    pub fn new() -> Self {
        Cache {
            buckets: HashMap::new(),
            stats: CacheStats {
                buckets: 0,
                columns: 0,
                rows: 0,
                inserts: 0,
                deletes: 0,
            },
        }
    }

    pub fn stats(&mut self) -> &CacheStats {
        // collect stats from buckets
        &self.stats
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
