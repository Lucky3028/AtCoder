use cli_test_dir::*;

const BIN: &'static str = "./jol_2022_2nd_a";

#[test]
fn sample1() {
    let test_dir = TestDir::new(BIN, "");
    let output = test_dir
        .cmd()
        .output_with_stdin(r#"7
joi
joig
ioi
READ
egoi
READ
READ
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"ioi
egoi
joig
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let test_dir = TestDir::new(BIN, "");
    let output = test_dir
        .cmd()
        .output_with_stdin(r#"20
one
READ
two
three
four
five
six
seven
READ
eight
nine
READ
ten
eleven
READ
READ
twelve
READ
READ
READ"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"one
seven
nine
eleven
ten
twelve
eight
six
"#);
    assert!(output.stderr_str().is_empty());
}
