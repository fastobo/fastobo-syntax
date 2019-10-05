
extern crate fastobo_syntax;
use fastobo_syntax::OboParser;
use fastobo_syntax::Rule;

macro_rules! test_parse {
    ($rule:ident, $input:literal) => ({
        match OboParser::parse(Rule::$rule, $input) {
            Ok(mut pairs) => assert_eq!(pairs.next().unwrap().as_str(), $input),
            Err(e) => panic!("could not parse {:?}:\n{}", $input, e),
        }
    })
}

macro_rules! test_not_parse {
    ($rule:ident, $input:literal) => ({
        match OboParser::parse(Rule::$rule, $input) {
            Err(_) => (),
            Ok(x) => panic!("successfully parsed {:?}:\n{}", $input, x),
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
    );
    test_parse!(
        HeaderClause,
        r#"synonymtypedef: systematic_synonym "Systematic synonym" EXACT"#
    );
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
fn synonym() {
    test_parse!(
        Synonym,
        r#""ciliated organ of asymmetry" EXACT TYPE [ZFIN:ZDB-PUB-050221-4]"#
    );
    test_parse!(
        Synonym,
        r#""ciliated organ of asymmetry" EXACT [ZFIN:ZDB-PUB-050221-4]"#
    );
    test_not_parse!(
        Synonym,
        r#""ciliated organ of asymmetry" EXACT_TYPE [ZFIN:ZDB-PUB-050221-4]"#
    );
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
