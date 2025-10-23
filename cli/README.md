CLI for Framework

Commands:
- new: interactively create a new project by copying `core/` into a new folder.
	- If there is no `core/` template, `new` will scaffold a minimal `core` crate so you can build from scratch.
- dev: run `cargo run` in the `core/` crate (starts the server).
- edit: update `core/src/engine/project.json` parent_folder interactively.

Usage:
- cd cli
- cargo run
