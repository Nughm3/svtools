# void-svtools
Basic wrappers for managing services for runit, just like `systemctl enable/disable <service>` or `rc-update add <service> <runlevel>`.

You can edit the included SVDIR and RUNSVDIR files to change the service directories. They default to where Void Linux stores service files.

- `sv-add`: symlink a service to runsvdir
- `sv-del`: remove a service symlink in runsvdir
- `sv-list`: display a listing of all services. Wraps `sv status <service>`, so you may need to have the appropriate permissions.

Licensed under the MIT License.
