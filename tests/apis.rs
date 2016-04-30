extern crate oxide;

#[test]
fn create_bucket() {
    let mut c = oxide::Cache::new();
    {
        let mut bb = oxide::BucketBuilder::new("foo");
        bb = bb.add_column(oxide::ColumnBuilder::UInt);
        c.new_bucket(bb).unwrap();
    };

    assert_eq!(true, c.has_bucket("foo"));
    let s = "foo".to_owned();
    assert_eq!(true, c.has_bucket(&s));

    assert_eq!(false, c.has_bucket("bar"));
}