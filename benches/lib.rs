#![feature(test)]

#[macro_use]
extern crate oxide;
extern crate rand;
extern crate test;

#[cfg(test)]
mod tests {
    use oxide;
    use rand;
    use test::Bencher;

    #[bench]
    fn bench_insert(b: &mut Bencher) {
        let mut c = oxide::Cache::new();
        {
            let mut bb = oxide::BucketBuilder::new("foo");
            bb = bb.add_column(oxide::ColumnBuilder::UInt);
            c.new_bucket(bb).unwrap();
        }
        b.iter(|| {
            c.bucket_mut("foo", |w| {
                let mut w = w.unwrap();
                w.insert(vals![rand::random::<usize>()]).unwrap();
            })
        });
    }
}
