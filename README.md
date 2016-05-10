## About
Oxide is a cache like library that stores tabular data values that are indexed with bitmap index, thus allows lookup in the fashion of "columnA = valueA and columnB = valueB". Usually you'd use database and/or SQL, occasionally it is better to do it in memory, hopefully.

This project is mostly to learn and experiment with Rust. The implementation uses roaring bitmap's Rust port [roaring-rs].

It also features a very random name.

## Usage

```rust
#[macro_use]
extern crate oxide;
use oxide::Match;

fn main() {
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new("my_bucket");
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        bb = bb.add_column(oxide::ColumnBuilder::Int);
        bb = bb.add_column(oxide::ColumnBuilder::OwnedStr);
        c.new_bucket(bb).unwrap();
    };

    let n = "my_bucket";
    let mut s = "hi";
    c.bucket_mut(n, |w| {
        let mut w = w.unwrap();
        let vals = vals![true, 1usize, s, -2isize, "yes".to_string()];
        match w.insert(vals) {
            Ok(_) => println!("inserted 1"),
            Err(e) => println!("{:?}", e),
        };

        let vals = vals![false, 2usize, s, -2isize, "nope".to_string()];
        match w.insert(vals) {
            Ok(_) => println!("inserted 2"),
            Err(e) => println!("{:?}", e),
        };

        let m = matches![true, Match::Any, "hi", -2isize, "yes".to_string()];

        if let Some(res) = w.find(&m).unwrap() {
            for r in res.iter() {
                print!("row:");
                for f in r.iter() {
                    print!(" {} ", f)
                }
                print!("\n");
            }
        } else {
            println!("no match");
        }
    });

}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)




[roaring-rs]: https://github.com/Nemo157/roaring-rs
