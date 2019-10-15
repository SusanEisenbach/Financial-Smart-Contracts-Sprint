use functional_tests::{checker::check, evaluator::eval, utils::parse_input};
use sprint_move::generator::Generator;
use sprint_parser::ast::contract::Visitor;
use std::{fmt::Display, fs::File, io::Read, path::Path};

fn test(module: impl Display, suite: &Path) {
    let mut input = String::new();

    input.push_str("//! account: alice, 1000000\n");
    input.push_str("//! account: bob, 1000000\n");
    input.push_str("//! account: chris, 1000000\n\n");

    input.push_str("//! new-transaction\n");
    input.push_str("//! sender: alice\n");

    input.push_str(&format!("{}\n", module));

    let mut suite = File::open(suite).unwrap();
    suite.read_to_string(&mut input).unwrap();

    let (config, directives, transactions) = parse_input(&input).unwrap();
    let log = eval(&config, &transactions).unwrap();

    if let Err(err) = check(&log, &directives) {
        println!("{}", log);
        panic!(err);
    }
}

#[test]
fn zero() {
    let suite = Path::new("tests/contracts/zero.test.mvir");

    let mut generator = Generator::new("Zero");
    generator.visit_zero();

    test(generator.contract(), suite);
}

#[test]
fn one() {
    let suite = Path::new("tests/contracts/one.test.mvir");

    let mut generator = Generator::new("One");
    generator.visit_one();

    test(generator.contract(), suite);
}