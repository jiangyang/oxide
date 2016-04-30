extern crate roaring;

mod errs;
mod token;
mod column;
mod value;
mod index;
mod matches;
mod pattern;
mod bucket;
mod cache;

pub use column::ColumnBuilder;
pub use value::Value;
pub use matches::Match;
pub use pattern::Pattern;
pub use bucket::{BucketBuilder, ReadHandle, WriteHandle};
pub use cache::Cache;

#[cfg(test)]
mod test {
    use super::bucket;

    #[test]
    fn it_works() {
        println!("yes!");
    }
}
