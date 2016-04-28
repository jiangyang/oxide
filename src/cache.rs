use std::collections::HashMap;

use errs::Error;
use column::ColumnRef;
use value::Value;
use matches::{Match, MatchResults};
use pattern::Pattern;
use bucket::{BucketBuilder, Bucket};

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

    pub fn insert(&mut self, bucket_name: &str, vals: Vec<Value<'c>>) -> Result<(), Error> {
        if let Some(b) = self.buckets.get_mut(bucket_name) {
            try!(b.insert(vals));
            Ok(())
        } else {
            Err(Error::InvalidBucket)
        }
    }

    pub fn insert_unique(&mut self, bucket_name: &str, vals: Vec<Value<'c>>) -> Result<bool, Error> {
        unimplemented!()
    }

    pub fn get_column_ref(&'c self, bucket_name: &str, column_num: usize) -> Option<ColumnRef<'c>> {
        if let Some(b) = self.buckets.get(bucket_name) {
            b.get_column_ref(column_num)
        } else {
            None
        }
    }

    pub fn find<'a>(&self,
                    bucket_name: &str,
                    pattern: &[Match<'a>])
                    -> Result<Option<MatchResults>, Error> {
        if let Some(b) = self.buckets.get(bucket_name) {
            if let Ok(Some(ref ids)) = b.find(pattern) {
                Ok(Some(b.get_by_ids(ids)))
            } else {
                Ok(None)
            }
        } else {
            Err(Error::InvalidBucket)
        }
    }

    pub fn delete<'a>(&mut self, bucket_name: &str, pattern: &[Match<'a>]) -> Result<usize, Error> {
        if let Some(b) = self.buckets.get_mut(bucket_name) {
            if let Ok(Some(ref ids)) = b.find(pattern) {
                Ok(b.delete_by_ids(ids))
            } else {
                Ok(0)
            }
        } else {
            Err(Error::InvalidBucket)
        }
    }

    pub fn find_pattern<'a>(&self,
                            bucket_name: &str,
                            pattern: &Pattern<'a>)
                            -> Result<Option<MatchResults>, Error> {
        if let Some(b) = self.buckets.get(bucket_name) {
            if let Ok(Some(ref ids)) = b.find_pattern(pattern) {
                Ok(Some(b.get_by_ids(ids)))
            } else {
                Ok(None)
            }
        } else {
            Err(Error::InvalidBucket)
        }
    }

    pub fn delete_pattern<'a>(&self,
                              bucket_name: &str,
                              pattern: &Pattern<'a>)
                              -> Result<usize, Error> {
        unimplemented!()
    }
}
