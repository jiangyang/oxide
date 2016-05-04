extern crate oxide;
use oxide::Value;
use oxide::ColumnBuilder;
use oxide::Pattern;

fn main() {
    let n = "my_bucket";
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new("my_bucket");
        bb = bb.add_column(ColumnBuilder::Boolean);
        bb = bb.add_column(ColumnBuilder::UInt);
        bb = bb.add_column(ColumnBuilder::Str);
        bb = bb.add_column(ColumnBuilder::OwnedStr);
        bb = bb.add_column(ColumnBuilder::Int);
        c.new_bucket(bb).unwrap();
    };

    let mut s = "hi";
    c.bucket_mut(n, |b| {
        let mut b = b.unwrap();

        let vals = vec![Value::Boolean(true),
                        Value::UInt(1),
                        Value::Str(s),
                        Value::OwnedStr(s.to_owned()),
                        Value::Int(99)];
        let r = b.insert(vals);
        match r {
            Ok(_) => println!("inserted 1"),
            Err(e) => println!("{:?}", e),
        };

        let vals = vec![Value::Boolean(false),
                        Value::UInt(2),
                        Value::Str(s),
                        Value::OwnedStr(s.to_owned()),
                        Value::Int(-99)];
        let r = b.insert(vals);
        match r {
            Ok(_) => println!("inserted 2"),
            Err(e) => println!("{:?}", e),
        };

        s = "bye";
        let vals = vec![Value::Boolean(true),
                        Value::UInt(3),
                        Value::Str(s),
                        Value::OwnedStr(s.to_owned()),
                        Value::Int(-100)];
        let r = b.insert(vals);
        match r {
            Ok(_) => println!("inserted 3"),
            Err(e) => println!("{:?}", e),
        };

        let c1 = b.get_column_ref(0).unwrap();
        let c2 = b.get_column_ref(1).unwrap();
        let c3 = b.get_column_ref(2).unwrap();
        let _c4 = b.get_column_ref(3).unwrap();
        let c5 = b.get_column_ref(4).unwrap();

        let m1_1 = Value::Boolean(true);
        let m1_2 = Value::Boolean(false);

        let m2_1 = Value::UInt(1);
        let m2_2 = Value::UInt(2);

        let m3 = Value::Str("hi");


        let m5_1 = Value::Int(-99);
        let m5_2 = Value::Int(99);

        let my_pattern = (Pattern::new(&c1, &m1_1).or(Pattern::new(&c1, &m1_2)))
                             .and(Pattern::new(&c2, &m2_1).or(Pattern::new(&c2, &m2_2)))
                             .and(Pattern::new(&c3, &m3))
                             .and(Pattern::new(&c5, &m5_1).or(Pattern::new(&c5, &m5_2)));

        if let Some(res) = b.find_pattern(&my_pattern).unwrap() {
            for r in res.iter() {
                print!("row: ");
                for f in r.iter() {
                    print!(" {} ", f)
                }
                print!("\n");
            }
        }
    });
}
