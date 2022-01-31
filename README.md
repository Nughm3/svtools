# void-svtools
Basic wrappers for managing services for runit, just like `systemctl enable/disable <service>` or `rc-update add <service> <runlevel>`.

For some tools you may need to specify the location of runit's service directories on your system by editing the constants in source files. It defaults to Void Linux's runsvdir locations.

- `sv-add`: symlink a service to runsvdir
- `sv-del`: remove a service symlink in runsvdir
- `sv-list`: display a listing of all services, showing whether they are enabled/disabled and if they are running. Wraps `sv status <service>`, so you may need to have the appropriate permissions.

Licensed under the MIT License.
