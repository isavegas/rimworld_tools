## Simple tools for RimWorld


### Planned features:
* Backup settings and mods. Initial goal.
* GUI? On the fence, here
* MacOS support. Maybe one day. I don't currently have a Mac.
* Linux support.
* Find install location based on Steam library configurations. Registry?
* DRM free version support.
* Workshop mod support for DRM free?
* Modpack system?
* Implement integration and end to end tests. Currently only testing internal
  semver module.

### Notes
* There are no libraries for iterating all steam libraries anywhere, so
  I'll have to implement one myself. Definitely something to implement
  after core functionality.
* Currently plenty of `#[allow(dead_code)]`. Partially to get rid of compiler
  warnings for what I'm currently working on, and partially because compiling
  `main.rs` will trip dead code warnings for unused code that is still exported
  in `lib.rs`. Understandable, yet aggravating nonetheless.