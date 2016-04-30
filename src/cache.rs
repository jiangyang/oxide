use std::collections::HashMap;
use std::fmt;

use errs::Error;
use bucket::{BucketBuilder, Bucket, ReadHandle, WriteHandle, BucketStats};

#[derive(Debug)]
pub struct CacheStats {
    buckets: HashMap<String, BucketStats>,
    columns: usize,
    inserts: usize,
    deletes: usize,
    rows: usize,
}

impl fmt::Display for CacheStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, bstats) in self.buckets.iter() {
            try!(writeln!(f, "==================================="));
            try!(writeln!(f, "{}", name));
            try!(write!(f, "{}", bstats));
        }
        try!(writeln!(f, "==================================="));
        try!(writeln!(f, "total buckets: {:>}", self.buckets.len()));
        try!(writeln!(f, "total columns: {:>}", self.columns));
        try!(writeln!(f, "total inserts: {:>}", self.inserts));
        try!(writeln!(f, "total deletes: {:>}", self.deletes));
        try!(writeln!(f, "total rows   : {:>}", self.rows));
        writeln!(f, "===================================")
    }
}

pub struct Cache<'c> {
    buckets: HashMap<String, Bucket<'c>>,
}

impl<'c> Cache<'c> {
    pub fn new() -> Self {
        Cache { buckets: HashMap::new() }
    }

    pub fn stats(&self) -> CacheStats {
        let mut s = CacheStats {
            buckets: HashMap::new(),
            columns: 0,
            inserts: 0,
            deletes: 0,
            rows: 0,
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

    pub fn new_bucket(&mut self, bb: BucketBuilder) -> Result<(), Error> {
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

    pub fn has_bucket(&self, bucket_name: &str) -> bool {
        self.buckets.contains_key(bucket_name)
    }

    pub fn bucket<F>(&self, bucket_name: &str, closure: F)
        where F: FnOnce(Option<ReadHandle>)
    {
        match self.buckets.get(bucket_name) {
            Some(b) => closure(Some(ReadHandle::new(b))),
            _ => closure(None),
        }
    }

    pub fn bucket_mut<F>(&mut self, bucket_name: &str, closure: F)
        where F: FnOnce(Option<WriteHandle>)
    {
        match self.buckets.get_mut(bucket_name) {
            Some(b) => {
                b.write().unwrap();
                closure(Some(WriteHandle::new(b)))
            }
            _ => closure(None),
        }
    }

    pub fn drop_bucket(&mut self, bucket_name: &str) {
        self.buckets.remove(bucket_name);
    }
}
