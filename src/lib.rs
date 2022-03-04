#[macro_use]
extern crate pest_derive;
extern crate pest;

use pest::error::Error;
use pest::error::ErrorVariant;
use pest::iterators::Pairs;

/// The OBO format version 1.4 lexer.
#[derive(Debug, Parser)]
#[grammar = "grammar.pest"]
pub struct OboLexer;

impl OboLexer {
    /// Tokenize an input string using the given production rule.
    ///
    /// This is basically a specialized version of [`pest::Parser::parse`]
    /// that only accepts [`Rule`], and does not need the `Parser` trait to
    /// be in scope.
    ///
    /// [`Rule`]: ./enum.Rule.html
    /// [`pest::Parser::parse`]: https://docs.rs/pest/latest/pest/trait.Parser.html
    pub fn tokenize(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        <Self as pest::Parser<Rule>>::parse(rule, input)
    }

    /// Tokenize the entirety of an input string.
    ///
    /// Contrary to [`OboLexer::tokenize`], this method will return an error if
    /// if the input is not parsed in its entirety.
    pub fn tokenize_all(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        match Self::tokenize(rule, input) {
            Ok(pairs) if pairs.as_str().len() == input.len() => Ok(pairs),
            Err(err) => Err(err),
            Ok(_) => {
                let variant = ErrorVariant::ParsingError {
                    positives: vec![rule],
                    negatives: vec![],
                };
                let span = pest::Span::new(input, 0, input.len()).unwrap();
                Err(Error::new_from_span(variant, span))
            }
        }
    }
}
