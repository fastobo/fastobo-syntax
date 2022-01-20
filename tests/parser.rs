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

macro_rules! test_not_parse {
    ($rule:ident, $input:literal) => {{
        match OboLexer::tokenize(Rule::$rule, $input) {
            Err(_) => (),
            Ok(x) => panic!("successfully parsed {:?}:\n{}", $input, x),
        }
    }};
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
    test_parse!(HeaderClause, r#"   format-version: 1.4"#);
}

#[test]
fn qualifier_list() {
    test_parse!(
        QualifierList,
        r#"{comment="NYBG:Dario_Cavaliere", comment="NYBG:Brandon_Sinn"}"#
    );

    test_parse!(
        QualifierList,
        r#"{comment="Depicted by field contains hematoxylin and eosin staining section of a stage 2 embryo. Scale bar = 100um"}"#
    )
}

#[test]
fn qualifier() {
    test_parse!(Qualifier, r#"comment="NYBG:Dario_Cavaliere""#)
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
fn term_frame() {
    test_parse!(TermFrame, "[Term]\nid: TEST:001\ndef: \"test item\" []\n");
    // with indentation
    test_parse!(TermFrame, "[Term]\n id: TEST:001\ndef: \"test item\" []\n");
    test_parse!(TermFrame, "[Term]\nid: TEST:001\n def: \"test item\" []\n");
    test_parse!(TermFrame, " [Term]\nid: TEST:001\ndef: \"test item\" []\n");
    // url id
    test_parse!(
        TermFrame,
        "[Term]\nid: http://example.com\ndef: \"test item\" []\n"
    );
    test_parse!(
        TermFrame,
        "[Term]\nid: http://example.com/\ndef: \"test item\" []\n"
    );
}

#[test]
fn term_clause_line() {
    test_parse!(TermClauseLine, "property_value: foaf:depiction http://purl.obolibrary.org/obo/plana/images/Stage2.png {comment=\"Depicted by field contains hematoxylin and eosin staining section of a stage 2 embryo. Scale bar = 100um\"}\n");
    test_parse!(TermClauseLine, "property_value: foaf:depiction http://purl.obolibrary.org/obo/plana/images/Stage2.png ! depicted by field\n");
}

#[test]
fn xref() {
    // Xref ID only
    test_parse!(Xref, r#"https://www.ncbi.nlm.nih.gov/pubmed/28402395"#);
    test_parse!(Xref, r#"DOI:10.1086/522843"#);
    test_parse!(Xref, r#"KEGG_COMPOUND"#);

    // Xref ID & description
    test_parse!(
        Xref,
        r#"DOI:10.1086/522843 "Gordon, Deborah. American Naturalist: Natural History Note. Dec. 2007""#
    );
}

#[test]
fn xreflist() {
    // Empty
    test_parse!(XrefList, r#"[]"#);

    // Single Xref, ID only
    test_parse!(
        XrefList,
        r#"[https://www.ncbi.nlm.nih.gov/pubmed/28402395]"#
    );
    test_parse!(XrefList, r#"[DOI:10.1086/522843]"#);
    test_parse!(XrefList, r#"[KEGG_COMPOUND]"#);

    // Single Xref, ID & description
    test_parse!(
        XrefList,
        r#"[DOI:10.1086/522843 "Gordon, Deborah. American Naturalist: Natural History Note. Dec. 2007"]"#
    );
}

#[test]
fn header_frame() {
    // with Unix endlines
    test_parse!(HeaderFrame, "format-version: 1.4\ndata-version: 1\n");
    // with Windows endlines
    test_parse!(HeaderFrame, "format-version: 1.4\r\ndata-version: 1\r\n");
}

#[test]
fn property_value() {
    test_parse!(
        PropertyValue,
        "foaf:depiction http://purl.obolibrary.org/obo/plana/images/Stage2.png"
    );
    test_parse!(
        ResourcePropertyValue,
        "foaf:depiction http://purl.obolibrary.org/obo/plana/images/Stage2.png"
    );
    test_not_parse!(
        LiteralPropertyValue,
        "foaf:depiction http://purl.obolibrary.org/obo/plana/images/Stage2.png"
    );
}

#[test]
fn iri() {
    test_parse!(Iri, "http://example.com");
    test_parse!(Iri, "http://example.com/");
    test_parse!(Iri, "http://example.com/with/a/path");
    test_parse!(Iri, "http://user:pass@example.com");
    test_parse!(Iri, "http://user:pass@example.com/");
    test_parse!(Iri, "http://user:pass@example.com/with/a/path");
}

#[test]
fn term_clause() {
    test_parse!(TermClause, "xref: http://example.com");
    test_parse!(TermClause, "xref: http://example.com/");
    test_parse!(TermClause, "xref: http://example.com/with/a/path");
    test_parse!(TermClause, "xref: http://user:pass@example.com");
    test_parse!(TermClause, "xref: http://user:pass@example.com/");
    test_parse!(TermClause, "xref: http://user:pass@example.com/with/a/path");
}
