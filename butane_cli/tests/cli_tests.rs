#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("../examples/getting_started/README.md")
        .case("../example/README.md");
}