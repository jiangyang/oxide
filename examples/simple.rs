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
        bb = bb.add_column(oxide::ColumnBuilder::Int);
        bb = bb.add_column(oxide::ColumnBuilder::OwnedStr);
        c.new_bucket(bb).unwrap();
    };

    c.bucket("nope", |r| {
        if let None = r {
            println!("no such bucket nope!");
            return;
        }
        println!("bucket nope exists!");
    });

    c.bucket_mut("nope", |w| {
        if let None = w {
            println!("no such bucket nope!");
            return;
        }
        println!("bucket nope exists!");
    });

    let n = "my_bucket";
    let mut s = "hi";
    c.bucket_mut(n, |w| {
        let mut w = w.unwrap();
        let vals = vec![
            Value::Boolean(true),
            Value::UInt(1),
            Value::Str(s),
            Value::Int(-2),
            Value::OwnedStr("yes".to_string())
        ];
        match w.insert(vals) {
            Ok(_) => println!("inserted 1"),
            Err(e) => println!("{:?}", e),
        };

        let vals = vec![
            Value::Boolean(false),
            Value::UInt(2),
            Value::Str(s),
            Value::Int(-2),
            Value::OwnedStr("nope".to_string())
        ];
        match w.insert(vals) {
            Ok(_) => println!("inserted 2"),
            Err(e) => println!("{:?}", e),
        };

        let m = vec![
            Match::Boolean(true),
            Match::Any,
            Match::Str("hi"),
            Match::Int(-2),
            Match::OwnedStr("yes".to_string())
        ];

        if let Some(res) = w.find(&m).unwrap() {
            println!("result is empty ? {}", res.is_empty());
            println!("result length is {}", res.len());

            for r in res.iter() {
                print!("row: ");
                for f in r.iter() {
                    print!(" {:?} ", f)
                }
                print!("\n");
            }
        } else {
            println!("no match");
        }
        s = "bye";
    });

    c.bucket_mut(n, |w| {
        let mut w = w.unwrap();
        let vals = vec![
            Value::Boolean(true), 
            Value::UInt(3), 
            Value::Str(s),
            Value::Int(-3),
            Value::OwnedStr("yes".to_string()) 
        ];
        match w.insert(vals) {
            Ok(_) => println!("inserted 3"),
            Err(e) => println!("{:?}", e),
        };
    });

    let p = vec![
        Match::Boolean(true), 
        Match::Any, 
        Match::Str("hi"),
        Match::Any,
        Match::OwnedStr("yes".to_string())
    ];

    c.bucket(n, |r| {
        let b = r.unwrap();

        if let Some(res) = b.find(&p).unwrap() {
            println!("result is empty ? {}", res.is_empty());
            println!("result length is {}", res.len());

            for r in res.iter() {
                print!("row: ");
                for f in r.iter() {
                    print!(" {:?} ", f)
                }
                print!("\n");
            }
        } else {
            println!("no match");
        }
    });

    c.bucket_mut(n, |w| {
        let mut b = w.unwrap();

        if let Ok(n) = b.delete(&p) {
            println!("deleted {} rows", n);
        }

        println!("now...{:?}", b.find(&p));
    });
}
