use std::fs;

use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args:&[&str], expected_file:&str)->TestResult{
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("headr")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// #[test]
// fn empty(){
//     run(&["tests/inputs/empty.txt"], "tests/inputs/empty.txt")
// }

// #[test]
// fn one(){
//     run(&["tests/inputs/one.txt"], "tests/inputs/one.txt")
// }

// #[test]
// fn two(){
//     run(&["tests/inputs/two.txt"], "tests/inputs/two.txt")
// }

// #[test]
// fn three(){
//     run(&["tests/inputs/three.txt"], "tests/inputs/three.txt")
// }

// #[test]
// fn ten(){
//     run(&["tests/inputs/ten.txt"], "tests/inputs/ten.txt")
// }


