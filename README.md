# pidfd_getfd
This crate provides a direct binding to the `pidfd_getfd` syscall, as well as
a slightly more convenient wrapper, `get_file_from_pidfd`. This also contains
an extension trait for [the `pidfd` crate](https://crates.io/crates/pidfd)
which provides access to `pidfd_getfd` via `PidFdExt::get_fd()`.

Please note that this crate has not been thoroughly tested. Viewer discretion
is advised.

## Example
```rust
use pidfd_getfd::{get_file_from_pidfd, GetFdFlags};
use std::io::Read;

let pidfd: RawFd = /* ... */;
let target_fd: RawFd = /* ... */;
let file = get_file_from_pidfd(pidfd, target_fd, GetFdFlags::empty());
let mut buf = Vec::new();
file.read_to_end(&mut buf)?;
println!("{:#?}", buf);
```
