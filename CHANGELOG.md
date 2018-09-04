# Changelog

## To be released

## v0.3.0-beta.0 - 2018-09-04

### Added

* Add a basic `Crossterm` backend

### Changed

* Remove `Group` and introduce `Layout` in its place
  - `Terminal` is no longer required to compute a layout
  - `Size` has been renamed `Constraint`
* Widgets are rendered on a `Frame` instead of a `Terminal` in order to
avoid mixing `draw` and `render` calls
* `draw` on `Terminal` expects a closure where the UI is built by rendering
widgets on the given `Frame`
* Update `Widget` trait
  - `draw` takes area by value
  - `render` takes a `Frame` instead of a `Terminal`
* All widgets use the consumable builder pattern
* `SelectableList` can have no selected item and the highlight symbol is hidden
in this case
* Remove markup langage inside `Paragraph`. `Paragraph` now expects an iterator
of `Text` items

## v0.2.3 - 2018-06-09

### Added

* Add `start_corner` option for `List`
* Add more text aligment options for `Paragraph`

## v0.2.2 - 2018-05-06

### Added

* `Terminal` implements `Debug`

### Changed

* Use `FnOnce` instead of `FnMut` in Group::render

## v0.2.1 - 2018-04-01

### Added

* Add `AlternateScreenBackend` in `termion` backend
* Add `TermionBackend::with_stdout` in order to let an user of the library
provides its own termion struct
* Add tests and documentation for `Buffer::pos_of`
* Remove leading whitespaces when wrapping text

### Fixed

* Fix `debug_assert` in `Buffer::pos_of`
* Pass the style of `SelectableList` to the underlying `List`
* Fix missing character when wrapping text
* Fix panic when specifying layout constraints

## v0.2.0 - 2017-12-26

### Added

* Add `MouseBackend` in `termion` backend to handle scroll and mouse events
* Add generic `Item` for items in a `List`

### Changed

* Rename `TermionBackend` to `RawBackend` (to distinguish it from the `MouseBackend`)
* Generic parameters for `List` to allow passing iterators as items
* Generic parameters for `Table` to allow using iterators as rows and header
* Generic parameters for `Tabs`
* Rename `border` bitflags to `Borders`

* Run latest `rustfmt` on all sources

### Removed

* Drop `log4rs` as a dev-dependencies in favor of `stderrlog`
