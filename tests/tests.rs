#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-spec.rs");
    t.pass("tests/02-i2c.rs");
    t.pass("tests/03-try-from.rs");
    t.pass("tests/04-full-cycle.rs");
}

