# Changelog

## Unreleased
No unreleased changes yet

## 0.2.1 - 2021-08-14
### Misc. changes
* Fix the `get_file_from_pidfd` example in the readme.
* Add a test for `get_file_from_pidfd`.

## 0.2.0 - 2021-08-14
### Additions
* Added the `nightly` feature, which implements `PidFdExt` for `std`'s `PidFd`.

### Breaking changes
* Renamed `PidFdExt::get_fd` to `PidFdExt::get_file` to better reflect its purpose.
* Renamed `GetfdFlags` to `GetFdFlags`.

### Misc. changes
* This crate should now display a more helpful error message when compiling on unsupported platforms (not Linux).
* Improved documentation and added more examples to the readme.
* Added compile-pass tests.

## 0.1.0 - 2021-08-05
Initial release
