
extern crate fastobo_syntax;
use fastobo_syntax::OboParser;
use fastobo_syntax::Rule;

macro_rules! test_parse {
    ($rule:ident, $input:literal) => ({
        match OboParser::parse(Rule::$rule, $input) {
            Ok(_) => (),
            Err(e) => panic!("could not parse {:?}:\n{}", $input, e),
        }
    })
}

#[test]
fn iso_date() {
    test_parse!(Iso8601DateTime, "2018-09-05T09:48:03Z");
    test_parse!(Iso8601DateTime, "2018-09-05T09:48:03.106Z");
}

#[test]
fn header_clause() {
    test_parse!(
        HeaderClause,
        "treat-xrefs-as-reverse-genus-differentia: TEST part_of something"
    )
}

#[test]
fn qualifier_list() {
    test_parse!(
        QualifierList,
        r#"{comment="NYBG:Dario_Cavaliere", comment="NYBG:Brandon_Sinn"}"#
    )
}

#[test]
fn qualifier() {
    test_parse!(
        Qualifier,
        r#"comment="NYBG:Dario_Cavaliere""#
    )
}

#[test]
fn xref() {
    // Xref ID only
    test_parse!(Xref, r#"https://www.ncbi.nlm.nih.gov/pubmed/28402395"#);
    test_parse!(Xref, r#"DOI:10.1086/522843"#);
    test_parse!(Xref, r#"KEGG_COMPOUND"#);

    // Xref ID & description
    test_parse!(Xref, r#"DOI:10.1086/522843 "Gordon, Deborah. American Naturalist: Natural History Note. Dec. 2007""#);
}

#[test]
fn xreflist() {
    // Empty
    test_parse!(XrefList, r#"[]"#);

    // Single Xref, ID only
    test_parse!(XrefList, r#"[https://www.ncbi.nlm.nih.gov/pubmed/28402395]"#);
    test_parse!(XrefList, r#"[DOI:10.1086/522843]"#);
    test_parse!(XrefList, r#"[KEGG_COMPOUND]"#);

    // Single Xref, ID & description
    test_parse!(XrefList, r#"[DOI:10.1086/522843 "Gordon, Deborah. American Naturalist: Natural History Note. Dec. 2007"]"#);
}
