extern crate fastobo_syntax;
use fastobo_syntax::OboLexer;
use fastobo_syntax::Rule;

macro_rules! test_parse {
    ($rule:ident, $input:literal) => {{
        match OboLexer::tokenize(Rule::$rule, $input) {
            Ok(mut pairs) => assert_eq!(pairs.next().unwrap().as_str(), $input),
            Err(e) => panic!("could not parse {:?}:\n{}", $input, e),
        }
    }};
}

#[test]
fn iso_datetime_utc() {
    test_parse!(Iso8601DateTime, "2018-09-05T09:48:03Z");
    test_parse!(Iso8601DateTime, "2018-09-05T09:48:03.106Z");
}

#[test]
fn iso_datetime_offset() {
    test_parse!(Iso8601DateTime, "2018-09-05T09:48:03+01:00");
    test_parse!(Iso8601DateTime, "2018-09-05T09:48:03-03:30");
    test_parse!(Iso8601DateTime, "2018-09-05T09:48:03−03:30");
}

#[test]
fn iso_datetime_compact() {
    test_parse!(Iso8601DateTime, "20180905T094803Z");
}

#[test]
fn creation_date_datetime() {
    test_parse!(TypedefClause, "creation_date: 2018-09-05T09:48:03Z");
}

#[test]
fn creation_date_date() {
    test_parse!(TypedefClause, "creation_date: 2018-09-05");
}
