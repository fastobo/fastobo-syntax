# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/fastobo/fastobo-syntax/compare/syntax/43b728e...HEAD


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
