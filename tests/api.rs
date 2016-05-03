extern crate oxide;

fn new_cache_with_bucket(name: &str) -> oxide::Cache {
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new(name);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        c.new_bucket(bb).unwrap();
    }
    c
}

#[test]
fn create_bucket() {
    let mut c = new_cache_with_bucket("foo");
    assert_eq!(true, c.has_bucket("foo"));
    let s = "foo".to_owned();
    assert_eq!(true, c.has_bucket(&s));
    assert_eq!(false, c.has_bucket("bar"));
}

#[test]
fn create_bucket_no_column() {
    let mut c = oxide::Cache::new();
    let mut bb = oxide::BucketBuilder::new("foo");
    if let Err(oxide::Error::NoColumn) = c.new_bucket(bb) {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn drop_bucket() {
    let n = "foo";
    let mut c = new_cache_with_bucket(n);
    assert_eq!(true, c.has_bucket(n));
    c.drop_bucket(n);
    assert_eq!(false, c.has_bucket(n));
}

#[test]
fn drop_invalid_bucket() {
    let n = "foo";
    let mut c = new_cache_with_bucket(n);
    assert_eq!(true, c.has_bucket(n));
    c.drop_bucket("bar");
    assert_eq!(true, c.has_bucket(n));
}

#[test]
fn insert() {
    let n = "foo";
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new(n);
        bb = bb.add_column(oxide::ColumnBuilder::Int);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        bb = bb.add_column(oxide::ColumnBuilder::OwnedStr);
        c.new_bucket(bb).unwrap();
    }

    c.bucket_mut(n, |w| {
        if let None = w {
            assert!(false);
        }
        let mut w = w.unwrap();

        use oxide::Value;
        // wrong number of things
        let v0 = vec![];
        if let Err(oxide::Error::WrongNumberOfValues(e, a)) = w.insert(v0) {
            assert_eq!(5, e);
            assert_eq!(0, a);
        } else {
            assert!(false);
        }

        let v1 = vec![Value::Int(1)];
        if let Err(oxide::Error::WrongNumberOfValues(e, a)) = w.insert(v1) {
            assert_eq!(5, e);
            assert_eq!(1, a);
        } else {
            assert!(false);
        }

        let v2 = vec![Value::Int(1), Value::UInt(2), Value::Boolean(false), Value::Boolean(true)];
        if let Err(oxide::Error::WrongNumberOfValues(e, a)) = w.insert(v2) {
            assert_eq!(5, e);
            assert_eq!(4, a);
        } else {
            assert!(false);
        }

        // wrong type of things
        let v3 = vec![Value::Int(1),
                      Value::UInt(2),
                      Value::Boolean(false),
                      Value::Boolean(true),
                      Value::Boolean(false)];
        if let Err(oxide::Error::WrongValueType(i)) = w.insert(v3) {
            assert_eq!(3, i);
        } else {
            assert!(false);
        }

        let v4 = vec![Value::Int(1),
                      Value::UInt(2),
                      Value::Boolean(false),
                      Value::Str("foo"),
                      Value::OwnedStr("boo".to_owned())];
        if let Ok(()) = w.insert(v4) {
            assert_eq!(1, w.rows());
            let stats = w.stats();
            assert_eq!(1, stats.inserts);
            assert_eq!(0, stats.deletes);
            assert_eq!(1, stats.rows);
            assert_eq!(5, stats.columns);
            assert_eq!(5, stats.index_stats.len());
            for c in stats.index_stats {
                assert_eq!(1, c.cardinality);
            }
        } else {
            assert!(false);
        }

        let v4 = vec![Value::Int(2),
                      Value::UInt(3),
                      Value::Boolean(true),
                      Value::Str("foo"),
                      Value::OwnedStr("boo".to_owned())];
        if let Ok(()) = w.insert(v4) {
            assert_eq!(2, w.rows());
            let stats = w.stats();
            assert_eq!(2, stats.inserts);
            assert_eq!(0, stats.deletes);
            assert_eq!(2, stats.rows);
            assert_eq!(5, stats.columns);
            assert_eq!(5, stats.index_stats.len());

            assert_eq!(2, stats.index_stats[0].cardinality);
            assert_eq!(2, stats.index_stats[1].cardinality);
            assert_eq!(2, stats.index_stats[2].cardinality);
            assert_eq!(1, stats.index_stats[3].cardinality);
            assert_eq!(1, stats.index_stats[4].cardinality);
        } else {
            assert!(false);
        }
    });

    c.bucket(n, |r| {
        if let None = r {
            assert!(false);
        }

        let r = r.unwrap();

        assert_eq!(2, r.rows());
        let stats = r.stats();
        assert_eq!(2, stats.inserts);
        assert_eq!(0, stats.deletes);
        assert_eq!(2, stats.rows);
        assert_eq!(5, stats.columns);
        assert_eq!(5, stats.index_stats.len());

        assert_eq!(2, stats.index_stats[0].cardinality);
        assert_eq!(2, stats.index_stats[1].cardinality);
        assert_eq!(2, stats.index_stats[2].cardinality);
        assert_eq!(1, stats.index_stats[3].cardinality);
        assert_eq!(1, stats.index_stats[4].cardinality);
    });
}

#[test]
fn insert_unique() {
    let n = "foo";
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new(n);
        bb = bb.add_column(oxide::ColumnBuilder::Int);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        bb = bb.add_column(oxide::ColumnBuilder::OwnedStr);
        c.new_bucket(bb).unwrap();
    }

    c.bucket_mut(n, |w| {
        if let None = w {
            assert!(false);
        }
        let mut w = w.unwrap();

        use oxide::Value;
        // wrong number of things
        let v0 = vec![];
        if let Err(oxide::Error::WrongNumberOfValues(e, a)) = w.insert_unique(v0) {
            assert_eq!(5, e);
            assert_eq!(0, a);
        } else {
            assert!(false);
        }

        let v1 = vec![Value::Int(1)];
        if let Err(oxide::Error::WrongNumberOfValues(e, a)) = w.insert_unique(v1) {
            assert_eq!(5, e);
            assert_eq!(1, a);
        } else {
            assert!(false);
        }

        let v2 = vec![Value::Int(1), Value::UInt(2), Value::Boolean(false), Value::Boolean(true)];
        if let Err(oxide::Error::WrongNumberOfValues(e, a)) = w.insert_unique(v2) {
            assert_eq!(5, e);
            assert_eq!(4, a);
        } else {
            assert!(false);
        }

        // wrong type of things
        let v3 = vec![Value::Int(1),
                      Value::UInt(2),
                      Value::Boolean(false),
                      Value::Boolean(true),
                      Value::Boolean(false)];
        if let Err(oxide::Error::WrongValueType(i)) = w.insert_unique(v3) {
            assert_eq!(3, i);
        } else {
            assert!(false);
        }

        let v4 = vec![Value::Int(1),
                      Value::UInt(2),
                      Value::Boolean(false),
                      Value::Str("foo"),
                      Value::OwnedStr("boo".to_owned())];
        if let Ok(true) = w.insert_unique(v4) {
            assert_eq!(1, w.rows());
            let stats = w.stats();
            assert_eq!(1, stats.inserts);
            assert_eq!(0, stats.deletes);
            assert_eq!(1, stats.rows);
            assert_eq!(5, stats.columns);
            assert_eq!(5, stats.index_stats.len());
            for c in stats.index_stats {
                assert_eq!(1, c.cardinality);
            }
        } else {
            assert!(false);
        }

        let v4 = vec![Value::Int(2),
                      Value::UInt(3),
                      Value::Boolean(true),
                      Value::Str("foo"),
                      Value::OwnedStr("boo".to_owned())];
        if let Ok(true) = w.insert_unique(v4) {
            assert_eq!(2, w.rows());
            let stats = w.stats();
            assert_eq!(2, stats.inserts);
            assert_eq!(0, stats.deletes);
            assert_eq!(2, stats.rows);
            assert_eq!(5, stats.columns);
            assert_eq!(5, stats.index_stats.len());

            assert_eq!(2, stats.index_stats[0].cardinality);
            assert_eq!(2, stats.index_stats[1].cardinality);
            assert_eq!(2, stats.index_stats[2].cardinality);
            assert_eq!(1, stats.index_stats[3].cardinality);
            assert_eq!(1, stats.index_stats[4].cardinality);
        } else {
            assert!(false);
        }

        let v5 = vec![Value::Int(2),
                      Value::UInt(3),
                      Value::Boolean(true),
                      Value::Str("foo"),
                      Value::OwnedStr("boo".to_owned())];
        if let Ok(false) = w.insert_unique(v5) {
            assert_eq!(2, w.rows());
            let stats = w.stats();
            assert_eq!(2, stats.inserts);
            assert_eq!(0, stats.deletes);
            assert_eq!(2, stats.rows);
            assert_eq!(5, stats.columns);
            assert_eq!(5, stats.index_stats.len());

            assert_eq!(2, stats.index_stats[0].cardinality);
            assert_eq!(2, stats.index_stats[1].cardinality);
            assert_eq!(2, stats.index_stats[2].cardinality);
            assert_eq!(1, stats.index_stats[3].cardinality);
            assert_eq!(1, stats.index_stats[4].cardinality);
        } else {
            assert!(false);
        }
    });

    c.bucket(n, |r| {
        if let None = r {
            assert!(false);
        }

        let r = r.unwrap();

        assert_eq!(2, r.rows());
        let stats = r.stats();
        assert_eq!(2, stats.inserts);
        assert_eq!(0, stats.deletes);
        assert_eq!(2, stats.rows);
        assert_eq!(5, stats.columns);
        assert_eq!(5, stats.index_stats.len());

        assert_eq!(2, stats.index_stats[0].cardinality);
        assert_eq!(2, stats.index_stats[1].cardinality);
        assert_eq!(2, stats.index_stats[2].cardinality);
        assert_eq!(1, stats.index_stats[3].cardinality);
        assert_eq!(1, stats.index_stats[4].cardinality);
    });
}

#[test]
fn find() {
    let n = "foo";
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new(n);
        bb = bb.add_column(oxide::ColumnBuilder::Int);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        bb = bb.add_column(oxide::ColumnBuilder::OwnedStr);
        c.new_bucket(bb).unwrap();
    }

    c.bucket_mut(n, |w| {
        let mut w = w.unwrap();
        use oxide::Value;
        w.insert(vec![Value::Int(-1),
                      Value::UInt(1),
                      Value::Boolean(false),
                      Value::Str("a"),
                      Value::OwnedStr("b".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-2),
                      Value::UInt(2),
                      Value::Boolean(true),
                      Value::Str("b"),
                      Value::OwnedStr("a".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-3),
                      Value::UInt(3),
                      Value::Boolean(false),
                      Value::Str("a"),
                      Value::OwnedStr("b".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-1),
                      Value::UInt(4),
                      Value::Boolean(false),
                      Value::Str("b"),
                      Value::OwnedStr("a".to_owned())])
         .unwrap();
    });

    c.bucket(n, |r| {
        if let None = r {
            assert!(false);
        }

        let r = r.unwrap();

        use oxide::Match;
        // wrong number of things
        let v0 = vec![];
        if let Err(oxide::Error::WrongNumberOfMatches(e, a)) = r.find(&v0) {
            assert_eq!(5, e);
            assert_eq!(0, a);
        } else {
            assert!(false);
        }

        let v1 = vec![Match::Int(1)];
        if let Err(oxide::Error::WrongNumberOfMatches(e, a)) = r.find(&v1) {
            assert_eq!(5, e);
            assert_eq!(1, a);
        } else {
            assert!(false);
        }

        let v2 = vec![Match::Int(1), Match::UInt(2), Match::Boolean(false), Match::Boolean(true)];
        if let Err(oxide::Error::WrongNumberOfMatches(e, a)) = r.find(&v2) {
            assert_eq!(5, e);
            assert_eq!(4, a);
        } else {
            assert!(false);
        }

        // wrong type of things
        let v3 = vec![Match::Int(1),
                      Match::UInt(2),
                      Match::Boolean(false),
                      Match::Boolean(true),
                      Match::Boolean(false)];
        if let Err(oxide::Error::WrongMatchType(i)) = r.find(&v3) {
            assert_eq!(3, i);
        } else {
            assert!(false);
        }

        // nothing to match
        if let Err(oxide::Error::NothingToMatch) = r.find(&vec![Match::Any; 5]) {
            assert!(true);
        } else {
            assert!(false);
        }

        // match 0
        let v4 = vec![Match::Int(-1),
                      Match::UInt(2),
                      Match::Boolean(false),
                      Match::Str("c"),
                      Match::OwnedStr("c".to_owned())];
        if let Ok(None) = r.find(&v4) {
            assert!(true);
        } else {
            assert!(false);
        }

        // match 1
        let v4 = vec![Match::Int(-1),
                      Match::UInt(4),
                      Match::Boolean(false),
                      Match::Str("b"),
                      Match::OwnedStr("a".to_owned())];
        if let Ok(Some(res)) = r.find(&v4) {
            assert_eq!(1, res.len());
        } else {
            assert!(false);
        }

        // match 2
        let v5 = vec![Match::Int(-1), Match::Any, Match::Any, Match::Any, Match::Any];
        if let Ok(Some(res)) = r.find(&v5) {
            assert_eq!(2, res.len());
        } else {
            assert!(false);
        }

        // pattern
        {
            use oxide::Value;
            use oxide::Pattern;

            let c1 = r.get_column_ref(0).unwrap();
            let c2 = r.get_column_ref(1).unwrap();
            let c3 = r.get_column_ref(2).unwrap();
            let c4 = r.get_column_ref(3).unwrap();
            let c5 = r.get_column_ref(4).unwrap();

            let m1_1 = Value::Int(-1);
            let m1_2 = Value::Int(-2);

            let m2_1 = Value::UInt(2);
            let m2_2 = Value::UInt(3);

            let m3_1 = Value::Boolean(true);
            let m3_2 = Value::Boolean(false);


            let m4 = Value::Str("b");

            let m5 = Value::OwnedStr("a".to_owned());

            // invalid pattern, type mis-match
            let my_pattern = (Pattern::new(&c1, &m1_1).or(Pattern::new(&c1, &m1_2)))
                                 .and(Pattern::new(&c2, &m2_1).or(Pattern::new(&c2, &m2_2)))
                                 .and(Pattern::new(&c3, &m4))
                                 .and(Pattern::new(&c5, &m5));

            if let Err(oxide::Error::InvalidColumnMatch) = r.find_pattern(&my_pattern) {
                assert!(true);
            } else {
                assert!(false);
            }

            let my_pattern = (Pattern::new(&c1, &m1_1).or(Pattern::new(&c1, &m1_2)))
                                 .and(Pattern::new(&c2, &m2_1).or(Pattern::new(&c2, &m2_2)))
                                 .and(Pattern::new(&c3, &m3_1).or(Pattern::new(&c3, &m3_2)))
                                 .and(Pattern::new(&c4, &m4))
                                 .and(Pattern::new(&c5, &m5));

            if let Ok(Some(res)) = r.find_pattern(&my_pattern) {
                assert_eq!(1, res.len());

                for r in res.iter() {
                    for f in r.iter() {
                        print!(" {:?} ", f)
                    }
                    print!("\n");
                }
            } else {
                assert!(false);
            }
        }

        assert_eq!(4, r.rows());
        let stats = r.stats();
        assert_eq!(4, stats.inserts);
        assert_eq!(0, stats.deletes);
        assert_eq!(4, stats.rows);
        assert_eq!(5, stats.columns);
        assert_eq!(5, stats.index_stats.len());

        assert_eq!(3, stats.index_stats[0].cardinality);
        assert_eq!(4, stats.index_stats[1].cardinality);
        assert_eq!(2, stats.index_stats[2].cardinality);
        assert_eq!(2, stats.index_stats[3].cardinality);
        assert_eq!(2, stats.index_stats[4].cardinality);
    });
}

#[test]
fn delete() {
    let n = "foo";
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new(n);
        bb = bb.add_column(oxide::ColumnBuilder::Int);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        bb = bb.add_column(oxide::ColumnBuilder::OwnedStr);
        c.new_bucket(bb).unwrap();
    }

    c.bucket_mut(n, |w| {
        let mut w = w.unwrap();
        use oxide::Value;
        w.insert(vec![Value::Int(-1),
                      Value::UInt(1),
                      Value::Boolean(false),
                      Value::Str("a"),
                      Value::OwnedStr("b".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-2),
                      Value::UInt(2),
                      Value::Boolean(true),
                      Value::Str("b"),
                      Value::OwnedStr("a".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-3),
                      Value::UInt(3),
                      Value::Boolean(false),
                      Value::Str("a"),
                      Value::OwnedStr("b".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-1),
                      Value::UInt(4),
                      Value::Boolean(false),
                      Value::Str("b"),
                      Value::OwnedStr("a".to_owned())])
         .unwrap();

        use oxide::Match;

        // match 1
        let v = vec![Match::Int(-1),
                     Match::UInt(4),
                     Match::Boolean(false),
                     Match::Str("b"),
                     Match::OwnedStr("a".to_owned())];
        if let Ok(n) = w.delete(&v) {
            assert_eq!(1, n);
            assert_eq!(3, w.rows());
        } else {
            assert!(false);
        }

        let stats = w.stats();
        assert_eq!(4, stats.inserts);
        assert_eq!(1, stats.deletes);
        assert_eq!(3, stats.rows);
        assert_eq!(5, stats.columns);
        assert_eq!(5, stats.index_stats.len());

        assert_eq!(3, stats.index_stats[0].cardinality);
        // FIXME: cardinality does not yet reduce on deletion
        assert_eq!(4, stats.index_stats[1].cardinality);
        assert_eq!(2, stats.index_stats[2].cardinality);
        assert_eq!(2, stats.index_stats[3].cardinality);
        assert_eq!(2, stats.index_stats[4].cardinality);

    });
}

#[test]
fn delete_pattern() {
    let n = "foo";
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new(n);
        bb = bb.add_column(oxide::ColumnBuilder::Int);
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        bb = bb.add_column(oxide::ColumnBuilder::Boolean);
        bb = bb.add_column(oxide::ColumnBuilder::Str);
        bb = bb.add_column(oxide::ColumnBuilder::OwnedStr);
        c.new_bucket(bb).unwrap();
    }

    c.bucket_mut(n, |w| {
        let mut w = w.unwrap();
        use oxide::Value;
        w.insert(vec![Value::Int(-1),
                      Value::UInt(1),
                      Value::Boolean(false),
                      Value::Str("a"),
                      Value::OwnedStr("b".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-2),
                      Value::UInt(2),
                      Value::Boolean(true),
                      Value::Str("b"),
                      Value::OwnedStr("a".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-3),
                      Value::UInt(3),
                      Value::Boolean(false),
                      Value::Str("a"),
                      Value::OwnedStr("b".to_owned())])
         .unwrap();
        w.insert(vec![Value::Int(-1),
                      Value::UInt(4),
                      Value::Boolean(false),
                      Value::Str("b"),
                      Value::OwnedStr("a".to_owned())])
         .unwrap();

        use oxide::Pattern;

        let c1 = w.get_column_ref(0).unwrap();
        let c2 = w.get_column_ref(1).unwrap();
        let c3 = w.get_column_ref(2).unwrap();
        let c4 = w.get_column_ref(3).unwrap();
        let c5 = w.get_column_ref(4).unwrap();

        let m1_1 = Value::Int(-1);
        let m1_2 = Value::Int(-2);

        let m2_1 = Value::UInt(2);
        let m2_2 = Value::UInt(3);

        let m3_1 = Value::Boolean(true);
        let m3_2 = Value::Boolean(false);

        let m4 = Value::Str("b");

        let m5 = Value::OwnedStr("a".to_owned());

        let my_pattern = (Pattern::new(&c1, &m1_1).or(Pattern::new(&c1, &m1_2)))
                             .and(Pattern::new(&c2, &m2_1).or(Pattern::new(&c2, &m2_2)))
                             .and(Pattern::new(&c3, &m3_1).or(Pattern::new(&c3, &m3_2)))
                             .and(Pattern::new(&c4, &m4))
                             .and(Pattern::new(&c5, &m5));

        if let Ok(n) = w.delete_pattern(&my_pattern) {
            assert_eq!(1, n);
            assert_eq!(3, w.rows());
        } else {
            assert!(false);
        }

        let stats = w.stats();
        assert_eq!(4, stats.inserts);
        assert_eq!(1, stats.deletes);
        assert_eq!(3, stats.rows);
        assert_eq!(5, stats.columns);
        assert_eq!(5, stats.index_stats.len());

        assert_eq!(3, stats.index_stats[0].cardinality);
        // FIXME: cardinality does not yet reduce on deletion
        assert_eq!(4, stats.index_stats[1].cardinality);
        assert_eq!(2, stats.index_stats[2].cardinality);
        assert_eq!(2, stats.index_stats[3].cardinality);
        assert_eq!(2, stats.index_stats[4].cardinality);

    });
}
