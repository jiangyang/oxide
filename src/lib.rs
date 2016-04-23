extern crate roaring;

mod errs;
mod value;
mod index;
mod matches;
mod pattern;
mod bucket;
mod cache;

pub use value::{Column, Value};
pub use matches::Match;
pub use pattern::Pattern;
pub use bucket::BucketBuilder;
pub use cache::Cache;

#[cfg(test)]
mod test {
    use super::bucket;

    #[test]
    fn it_works() {
      println!("yes!");
    }
}
