# `fastobo-syntax` [![Star me](https://img.shields.io/github/stars/fastobo/fastobo-syntax.svg?style=social&label=Star&maxAge=3600)](https://github.com/fastobo/fastobo-syntax/stargazers)

*PEG Syntax and pest lexer for the OBO flat file format 1.4.*

[![Actions](https://img.shields.io/github/workflow/status/fastobo/fastobo-syntax/Test?style=flat-square&maxAge=600)](https://github.com/fastobo/fastobo-syntax/actions)
[![TravisCI](https://img.shields.io/travis/com/fastobo/fastobo-syntax/master.svg?maxAge=600&style=flat-square)](https://travis-ci.com/fastobo/fastobo-syntax/branches)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo-syntax/)
[![Crate](https://img.shields.io/crates/v/fastobo-syntax.svg?maxAge=600&style=flat-square)](https://crates.io/crates/fastobo-syntax)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/fastobo/latest/fastobo/parser/)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo-syntax/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/fastobo/fastobo-syntax.svg?style=flat-square)](https://github.com/fastobo/fastobo-syntax/issues)
[![DOI](https://img.shields.io/badge/doi-10.7490%2Ff1000research.1117405.1-brightgreen?style=flat-square&maxAge=31536000)](https://f1000research.com/posters/8-1500)


## Overview

This library is a strict implementation of the [OBO flat file format 1.4](http://owlcollab.github.io/oboformat/doc/obo-syntax.html)
syntax using the [`pest`](https://pest.rs/) parser generator. It was outsourced from
[`fastobo`](https://github.com/fastobo/fastobo/) to reduce compilation time, since deriving the `OboLexer` from
[`grammar.pest`](https://github.com/fastobo/fastobo-syntax/blob/master/src/grammar.pest) takes some time.

The lexer itself is reexported in [`fastobo::parser`](https://docs.rs/fastobo/latest/fastobo/parser/),
so there is probably no need to depend on this crate directly.


## Strictness

The syntax is a strict implementation of the 1.4 format, with the following exceptions:

* `property_value` clauses can have a value with is not quote-enclosed. This is a workaround
  to support some ontology files using `obo2owl` or the `owlapi` to generate their OBO
  counterpart, which does not quote-enclose property values
  ([owlcs/owlapi#833](https://github.com/owlcs/owlapi/pull/833)).
* ISO-8601 datetimes can only be parsed from the canonical format (`YYYY-MM-DDTHH:MM:SS`)
  with an optional timezone. Week dates and calendar dates are not supported.
* Dates in `creation_date` clauses can be either full ISO-8601 datetimes
  (as recommended by the format 1.4 specifications) or simply ISO-8601 dates,
  which is suggested by the format 1.4 guide (albeit non-normative).


## See also

* [`fastobo`](https://crates.io/crates/fastobo): Abstract Syntax Tree and data
  structures for the OBO format version 1.4.
* [`fastobo-py`](https://pypi.org/project/fastobo/): Idiomatic Python bindings
  to the `fastobo` crate.


## Feedback

Found a bug ? Have an enhancement request ? Head over to the
[GitHub issue tracker](https://github.com/fastobo/fastobo-syntax/issues) of the project if
you need to report or ask something. If you are filling in on a bug, please include as much
information as you can about the issue, and try to recreate the same bug in a simple, easily
reproducible situation.


## About

This project was developed by [Martin Larralde](https://github.com/althonos)
as part of a Master's Degree internship in the [BBOP team](http://berkeleybop.org/) of the
[Lawrence Berkeley National Laboratory](https://www.lbl.gov/), under the supervision of
[Chris Mungall](http://biosciences.lbl.gov/profiles/chris-mungall/). Please cite this project as:

*Larralde M.* **Developing Python and Rust libraries to improve the ontology ecosystem**
*\[version 1; not peer reviewed\].* F1000Research 2019, 8(ISCB Comm J):1500 (poster)
([https://doi.org/10.7490/f1000research.1117405.1](https://doi.org/10.7490/f1000research.1117405.1))
