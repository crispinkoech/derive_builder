use derive_builder::Builder;

#[derive(Builder)]
struct TestStruct<'t> {
    field_a: i8,
    field_b: String,
    field_c: Option<&'t str>,
}

fn main() {
    let test_case = TestStruct::builder()
        .field_a(1)
        .field_b("example".to_string());

    assert_eq!(test_case.field_a, Some(1));
    assert_eq!(test_case.field_b, Some("example".to_string()));
    assert_eq!(test_case.field_c, None);
}
