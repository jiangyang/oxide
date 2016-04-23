extern crate oxide;
use oxide::Value;
use oxide::Match;
use oxide::Column;
use oxide::Pattern;

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

    // TODO:
    // let c1 = Column::UInt;
    // let c2 = Column::Boolean;
    // let c3 = Column::UInt;
    
    // let m1_1 = Match::UInt(1);
    // let m1_2 = Match::UInt(2);
    
    // let m2_1 = Match::Boolean(true);
    // let m2_2 = Match::Boolean(false);
    
    // let m3 = Match::UInt(99);
    
    // let my_pattern = Pattern::new(&c1, &m1_1).or(Pattern::new(&c1, &m1_2))
    //     .and(Pattern::new(&c2, &m2_1).or(Pattern::new(&c2, &m2_2)))
    //     .and(Pattern::new(&c3, &m3));
        
    // println!("{:?}", my_pattern);

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
