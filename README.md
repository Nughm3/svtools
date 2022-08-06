# svtools

Basic wrappers for managing services for `runit`, just like `systemctl enable/disable <service>` or `rc-update add <service> <runlevel>`. Implemented in [Rust](//rust-lang.org).

Primarily designed for [Void Linux](//voidlinux.org).

- `sv-add`: symlink a service to runsvdir
- `sv-del`: remove a service symlink in runsvdir
- `sv-list`: display a listing of all services. Wraps `sv status <service>`, so you may need to have the appropriate permissions.

## Build/Install

```bash
git clone https://github.com/Nughm3/svtools
cd svtools
cargo install --path .
```

Cargo will install `svtools` as separate binaries.

### Licensing

`svtools` is licensed under the [MIT License](//opensource.org/licenses/MIT).
