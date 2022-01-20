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
fn term_frame() {
    test_parse!(EntityFrame, "[Term]\nid: TST:001\n");
    test_parse!(EntityFrame, "! irrelevant\n[Term]\nid: TST:001\n");
    test_parse!(EntityFrame, "[Term]\n! irrelevant\nid: TST:001\n");
    test_parse!(EntityFrame, "[Term]\nid: TST:001\n! irrelevant\n");
}

#[test]
fn header_frame() {
    test_parse!(HeaderFrame, "format-version: 1,4\n! this is a comment\n");
    test_parse!(HeaderFrame, "! this is a comment\nformat-version: 1,4\n");
    test_parse!(OboDoc, "format-version: 1,4\n! this is a comment\n");
    test_parse!(OboDoc, "! this is a comment\nformat-version: 1,4\n");
}

#[test]
fn typedef_frame() {
    test_parse!(EntityFrame, "[Typedef]\nid: TST:001\n");
    test_parse!(EntityFrame, "! irrelevant\n[Typedef]\nid: TST:001\n");
    test_parse!(EntityFrame, "[Typedef]\n! irrelevant\nid: TST:001\n");
    test_parse!(EntityFrame, "[Typedef]\nid: TST:001\n! irrelevant\n");
}

#[test]
fn instance_frame() {
    test_parse!(EntityFrame, "[Instance]\nid: TST:001\n");
    test_parse!(EntityFrame, "! irrelevant\n[Term]\nid: TST:001\n");
    test_parse!(EntityFrame, "[Instance]\n! irrelevant\nid: TST:001\n");
    test_parse!(EntityFrame, "[Instance]\nid: TST:001\n! irrelevant\n");
}
