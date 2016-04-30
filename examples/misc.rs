extern crate oxide;
use oxide::Value;
use oxide::Match;

fn main() {
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new("foo");
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        c.new_bucket(bb).unwrap();
    };

    {
        let mut bb = oxide::BucketBuilder::new("bar");
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        c.new_bucket(bb).unwrap();
    };

    let s = "hi".to_string();
    c.bucket_mut("foo", |w| {
        let mut w = w.unwrap();
        w.insert(vec![Value::Boolean(true), Value::UInt(1), Value::Str(&s.clone())]).unwrap();
        w.insert(vec![Value::Boolean(true), Value::UInt(2), Value::Str(&s.clone())]).unwrap();
        w.insert(vec![Value::Boolean(true), Value::UInt(3), Value::Str(&s.clone())]).unwrap();

        // wont compile
        // c.bucket("bar", |w| {
        //     let mut w = w.unwrap();
        //     w.find(&vec![Match::Boolean(true), Match::Any, Match::Str("hi")]).unwrap();
        // });
    });

    c.bucket_mut("bar", |w| {
        let w = w.unwrap();
        w.insert(vec![Value::UInt(1), Value::Str(&s.clone()), Value::Boolean(true)]).unwrap();
    });

    println!("{:?}", c.stats());
}
