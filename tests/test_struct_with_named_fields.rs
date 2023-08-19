use derive_builder::Builder;

#[derive(Builder)]
struct TestStruct<'t> {
    field_a: i8,
    field_b: String,
    field_c: &'t str,
    field_d: Vec<i8>,
    field_e: Option<String>,
    field_f: Option<&'t str>,
}

fn main() {
    test_fully_filled_struct();
    test_partially_filled_struct();
}

fn test_fully_filled_struct() {
    let test_case = TestStruct::builder()
        .field_a(1)
        .field_b("example".to_string())
        .field_c("string slice")
        .field_d(10)
        .field_e("another string slice".to_string())
        .field_f("yet another string slice")
        .build()
        .unwrap();

    assert_eq!(test_case.field_a, 1);
    assert_eq!(test_case.field_b, "example".to_string());
    assert_eq!(test_case.field_c, "string slice");
    assert_eq!(test_case.field_d, vec![10]);
    assert_eq!(test_case.field_e, Some("another string slice".to_string()));
    assert_eq!(test_case.field_f, Some("yet another string slice"));
}

fn test_partially_filled_struct() {
    let test_case = TestStruct::builder()
        .field_a(1)
        .field_b("example".to_string())
        .field_c("string slice")
        .build()
        .unwrap();

    assert_eq!(test_case.field_a, 1);
    assert_eq!(test_case.field_b, "example".to_string());
    assert_eq!(test_case.field_c, "string slice");
    assert_eq!(test_case.field_d, vec![]);
    assert_eq!(test_case.field_e, None);
    assert_eq!(test_case.field_f, None);
}
