# pidfd_getfd

[![pidfd_getfd on crates.io](https://img.shields.io/crates/v/pidfd-getfd)](https://crates.io/crates/pidfd_getfd)
[![Latest documentation on docs.rs](https://docs.rs/pidfd_getfd/badge.svg)](https://docs.rs/pidfd_getfd)
![License information for pidfd-getfd](https://img.shields.io/crates/l/pidfd-getfd)

This crate provides a direct binding to the [`pidfd_getfd`] syscall along with
a slightly more convenient wrapper, `get_file_from_pidfd`. This also contains
an extension trait for [`pidfd::PidFd`] and [`std`'s `PidFd`] (currently only
available on nightly rustc) which provides access to `get_file_from_pidfd` via
`PidFdExt::get_file()`.

[`pidfd_getfd`]: https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html
[`pidfd::PidFd`]: https://docs.rs/pidfd/0.2.4/pidfd/struct.PidFd.html
[`std`'s `PidFd`]: https://doc.rust-lang.org/nightly/std/os/linux/process/struct.PidFd.html

Note that `pidfd`s are currently only supported on Linux 5.6 or later, thus this crate
will only work on Linux. If any other platform gains support for `pidfd`s, please
let me know through an issue or pull request!

Please note that this crate has not been thoroughly tested. Viewer discretion
is advised.

## Example
```rust
use pidfd_getfd::{get_file_from_pidfd, GetFdFlags};
use std::{
    io::{self, Read},
    os::unix::prelude::RawFd,
};

let pidfd: RawFd = /* ... */;
let target_fd: RawFd = /* ... */;
let mut file = get_file_from_pidfd(pidfd, target_fd, GetFdFlags::empty())?;
let mut buf = Vec::new();
file.read_to_end(&mut buf)?;
println!("{:#?}", buf);
Ok(())
```

Using `pidfd`:
```rust
use pidfd::PidFd;
use pidfd_getfd::{GetFdFlags, PidFdExt};
use std::process::Command;

let child = Command::new("/usr/bin/foo").spawn().expect("failed to run `foo`");
let pidfd = PidFd::from_std_checked(&child)?;
let file_from_child = pidfd.get_file(1, GetFdFlags::empty())?;
```

Using nightly rustc:
```rust
#![feature(linux_pidfd)]

use pidfd_getfd::{GetFdFlags, PidFdExt};
use std::{
    os::linux::process::{ChildExt, CommandExt},
    process::Command,
};

let child = Command::new("/usr/bin/foo")
    .create_pidfd(true)
    .spawn()
    .expect("failed to run `foo`");

let file_from_child = child.pidfd()?.get_file(1, GetFdFlags::empty())?;
```
