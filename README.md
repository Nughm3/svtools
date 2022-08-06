# svtools

Basic wrappers for managing services for runit, just like `systemctl enable/disable <service>` or `rc-update add <service> <runlevel>`.

If you are not on [Void Linux](//voidlinux.org) make sure to change the constants RUNSVDIR and SVDIR in `sv-*/src/main.rs` files.

- `sv-add`: symlink a service to runsvdir
- `sv-del`: remove a service symlink in runsvdir
- `sv-list`: display a listing of all services. Wraps `sv status <service>`, so you may need to have the appropriate permissions.

Licensed under the MIT License.
