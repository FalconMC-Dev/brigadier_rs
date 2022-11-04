# Changelog

## Unpublished

### Added

- Add simple unit test.

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
