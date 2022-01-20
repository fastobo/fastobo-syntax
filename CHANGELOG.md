# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
[Unreleased]: https://github.com/fastobo/fastobo-syntax/compare/v0.7.0...HEAD

## [v0.7.1] - 2022-01-20
[v0.7.1]: https://github.com/fastobo/fastobo-syntax/compare/v0.7.0...v0.7.1
### Changed
- Move `Iso8601Timezone` into the `Iso8601Time` production rule.

## [v0.7.0] - 2022-01-20
[v0.7.0]: https://github.com/fastobo/fastobo-syntax/compare/v0.6.2...v0.7.0
### Changed
- `creation_date` clauses can now take an `Iso8601Date` without the time specifier.
### Fixed
- `Iso8601TimeZoneSign` now accepts the mathematical minus sign (`−`).

## [v0.6.2] - 2020-09-28
[v0.6.2]: https://github.com/fastobo/fastobo-syntax/compare/v0.6.1...v0.6.2
### Fixed
- Parser crashing on URLs missing a trailing slash after the host component
  (e.g. `http://example.com`).

## [v0.6.1] - 2020-08-28
[v0.6.1]: https://github.com/fastobo/fastobo-syntax/compare/v0.6.0...v0.6.1
### Fixed
- Parser crashing on `PropertyValue` followed by a qualifier list.

## [v0.6.0] - 2020-08-28
[v0.6.0]: https://github.com/fastobo/fastobo-syntax/compare/v0.5.0...v0.6.0
### Added
- `Definition` production rule to match value of *def* clauses.
### Changed
- Renamed `OboLexer::parse` to `OboLexer::tokenize`.
- `PropertyValue` production rule now has two subrules `ResourcePropertyValue` and `LiteralPropertyValue` to aid constructing enum variants.
- `LiteralPropertyValue` accepts any identifier as a datatype, not just `xsd`-prefixed datatypes.

## [v0.5.0] - 2020-07-23
[v0.5.0]: https://github.com/fastobo/fastobo-syntax/compare/v0.4.0...v0.5.0
### Changed
- Renamed `OboParser` struct to `OboLexer`.

## [v0.4.0] - 2020-07-20
[v0.4.0]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.8...v0.4.0
### Added
- Support for ignoring full comment lines in entity and frames
  ([#1](https://github.com/fastobo/fastobo-syntax/issues/1)).

## [v0.3.8] - 2020-01-23
[v0.3.8]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.7...v0.3.8
### Fixed
- Made `grammar.pest` support indented frames and clauses (event unconsistently).

## [v0.3.7] - 2020-01-18
[v0.3.7]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.6...v0.3.7
### Fixed
- Slightly improve parsing of quoted strings in `grammar.pest`.
- Fixed Windows newlines not being recognized.

## [v0.3.6] - 2019-10-05
[v0.3.6]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.5...v0.3.6
### Fixed
- `SynonymScope` rule change breaking `synonymtypedef` header clauses.

## [v0.3.5] - 2019-10-05 - **YANKED**
[v0.3.5]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.4...v0.3.5
### Fixed
- `Synonym` production rule breaking on valid synonyms with `v0.3.4`.

## [v0.3.4] - 2019-10-05 - **YANKED**
[v0.3.4]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.3...v0.3.4
### Added
- Added BOSC 2019 poster reference to `README.md`.
### Fixed
- `Synonym` production rule parsing synonym types without whitespace after scope.

## [v0.3.3] - 2019-07-23
[v0.3.3]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.2...v0.3.3
### Changed
- `Iso8601DateTime` rule to allow parsing `Iso8601Fraction` with `f32::from_str`.

## [v0.3.2] - 2019-07-23
[v0.3.2]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.1...v0.3.2
### Fixed
- `Iso8601DateTime` rule support for dates with fractional seconds components.

## [v0.3.1] - 2019-06-04
[v0.3.1]: https://github.com/fastobo/fastobo-syntax/compare/v0.3.0...v0.3.1
### Fixed
- `XrefId` rule allowing whitespaces, causing a bug with some Xref lists.

## [v0.3.0] - 2019-05-14
[v0.3.0]: https://github.com/fastobo/fastobo-syntax/compare/v0.2.1...v0.3.0
### Added
- Support for `is_asymmetric` typedef clause in grammar.

## [v0.2.1] - 2019-05-09
[v0.2.1]: https://github.com/fastobo/fastobo-syntax/compare/43b728e...v0.2.1
### Changed
- Outsourced crate to `fastobo/fastobo-syntax` GitHub repository.

## v0.2.0 - 2019-05-06
### Added
- Explicit support for `namespace-id-rule` header clause.
### Changed
- Use builtin `pest` whitespace interpolation in `grammar.pest`.
### Fixed
- Allow newlines between clause lines and between frames.
- Fixed invalid whitespace in `TreatXrefsAsRelationshipTag` causing parser to fail
  on some `treat-xrefs-as-relationship` header clauses.

## v0.1.1 - 2019-04-12
### Fixed
- Fix invalid header values being successfully parsed into `Unreserved` nonetheless.

## v0.1.0 - 2019-03-30

Initial release.
