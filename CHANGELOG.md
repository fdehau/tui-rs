# Changelog

## Unreleased

### Added

* Add `MouseBackend` in `termion` backend to handle scroll and mouse events
* Add generic `Item` for items in a `List`

### Changed

* Rename `TermionBackend` to `RawBackend` (to distinguish it from the `MouseBackend`)
* Generic parameters for `List` to allow passing iterators as items
* Generic parameters for `Table` to allow using iterators as rows and header
* Generic parameters for `Tabs`

* Run latest `rustfmt` on all sources

### Removed

* Drop `log4rs` as a dev-dependencies in favor of `stderrlog`
