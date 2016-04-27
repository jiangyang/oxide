extern crate oxide;
use oxide::Value;
use oxide::Match;
use oxide::ColumnBuilder;
use oxide::Pattern;

fn main() {
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new("my_bucket");
        bb = bb.add_column(ColumnBuilder::Boolean);
        bb = bb.add_column(ColumnBuilder::UInt);
        bb = bb.add_column(ColumnBuilder::Str);
        c.new_bucket(bb).unwrap();
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

    // borrow cache in the scope for pattern
    // because columnRef borrows the cache
    {   
        let c = &c;
        let c1 = c.get_column_ref("my_bucket", 0).unwrap();
        let c2 = c.get_column_ref("my_bucket", 1).unwrap();
        let c3 = c.get_column_ref("my_bucket", 2).unwrap();
        
        let m1_1 = Match::Boolean(true);
        let m1_2 = Match::Boolean(false);

        let m2_1 = Match::UInt(1);
        let m2_2 = Match::UInt(2);
        
        let m3 = Match::Str("hi");
        
        let my_pattern = Pattern::new(&c1, &m1_1).or(Pattern::new(&c1, &m1_2))
            .and(Pattern::new(&c2, &m2_1).or(Pattern::new(&c2, &m2_2)))
            .and(Pattern::new(&c3, &m3));
            
        println!("{:?}", my_pattern);

        if let Some(res) = c.find_pattern("my_bucket", &my_pattern).unwrap() {
            println!("result is empty ? {}", res.is_empty());
            println!("result length is {}", res.len());

            for r in res.iter() {
                print!("row: ");
                for f in r.iter() {
                    print!(" {:?} ", f)
                }
                print!("\n");
            }
        }
    }

    let p = vec![Match::Boolean(true), Match::Any, Match::Str("hi")];
    if let Ok(n) = c.delete("my_bucket", &p) {
        println!("deleted {} rows", n);
    }

    println!("now...{:?}" , c.find("my_bucket", &p));
}
