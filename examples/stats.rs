extern crate oxide;
use oxide::Value;
use oxide::Match;

fn main() {
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new("my_bucket");
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        c.new_bucket(bb).unwrap();
    };

    let n = "my_bucket";
    let s = "hi";
    c.bucket_mut(n, |w| {
        let mut w = w.unwrap();
        w.insert(vec![Value::Boolean(true), Value::UInt(1), Value::Str(s)]).unwrap();
        w.insert(vec![Value::Boolean(false), Value::UInt(2), Value::Str(s)]).unwrap();

        let m = vec![Match::Boolean(true), Match::Any, Match::Str("hi")];
        w.delete(&m).unwrap();

        w.insert(vec![Value::Boolean(true), Value::UInt(1), Value::Str(s)]).unwrap();
        w.insert(vec![Value::Boolean(true), Value::UInt(2), Value::Str(s)]).unwrap();
        w.insert(vec![Value::Boolean(true), Value::UInt(3), Value::Str(s)]).unwrap();

    });
    println!("{:?}", c.stats());
}
