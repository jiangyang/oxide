extern crate oxide;
use oxide::Value;
use oxide::Match;

fn main() {
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new("my_bucket");
        bb = bb.add_column(oxide::Column::Boolean);
        bb = bb.add_column(oxide::Column::UInt);
        bb = bb.add_column(oxide::Column::Str);
        c.new_bucket(bb)
    };

    let mut s = "hi";

    let vals = vec![Value::Boolean(true), Value::UInt(1), Value::Str(s)];
    let r = c.insert("my_bucket", vals);
    match r {
        Ok(_) => println!("success!"),
        Err(e) => println!("{:?}", e),
    };

    let vals = vec![Value::Boolean(false), Value::UInt(2), Value::Str(s)];
    let r = c.insert("my_bucket", vals);
    match r {
        Ok(_) => println!("success!"),
        Err(e) => println!("{:?}", e),
    };

    s = "bye";
    let vals = vec![Value::Boolean(true), Value::UInt(3), Value::Str(s)];
    let r = c.insert("my_bucket", vals);
    match r {
        Ok(_) => println!("success!"),
        Err(e) => println!("{:?}", e),
    };

    let p = vec![Match::Boolean(true), Match::Any, Match::Str("hi")];
    if let Some(res) = c.find("my_bucket", &p).unwrap() {
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

    if let Ok(n) = c.delete("my_bucket", &p) {
        println!("deleted {} rows", n);
    }

    println!("now...{:?}" , c.find("my_bucket", &p));

}
