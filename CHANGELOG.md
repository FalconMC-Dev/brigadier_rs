# Changelog

## Unpublished

### Added

- Add unit test for parser.
- Add unit test for usage.
- New field `name` for `BoolArgument`.
- New field `name` for `NumberArgument`.
- Add `SingleUsage` trait.
- Add `MultipleUsage` trait.
- Add `IntoMultipleUsage` trait.
- Add `ChildUsage` trait.
- Add `Prefix` type and `prefix` fn.
- Add `Chain` type (from `chain` fn on `MultipleUsage`).
- Implement Usage traits for `BoolArgument`, `LiteralArgument` and `NumberArgument`.
- Implement Usage traits for `CommandThen`, `ThenWrapper` and `LiteralThen`.
- Implement Usage traits for `DefaultExecutor`, `LiteralExecutor`, `LiteralThenExecutor` and `ThenExecutor`.
- Add `UsagePrint`.

### Changed

- Return value is now any `U` instead of `bool`.

### Fixed

- `NumberAgument` now returns a `Failure` when out of bounds.

---

## Release 0.1.0

### Added

- Expose the `ArgumentMarkerDefaultImpl` trait.
- `boolean()` is now public (was hidden in doc).
- Add crates badge to `README.md`.

### Changed

- Renamed `IntegerArgument` to `NumberArgument`.
- Add documentation for existing public types.
- Implement `ContextError` for `CommandError`.

### Fixed
