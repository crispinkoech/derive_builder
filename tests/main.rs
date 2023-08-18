use trybuild::TestCases;

#[test]
fn it_compiles_when_annotating_struct_with_named_fields() {
    TestCases::new().pass("tests/test_struct_with_named_fields.rs")
}
