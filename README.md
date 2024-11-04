# *ced* : Code EDitor

*ced* is tui text-editor written in Rust.

## Known issue

- Backspace? Delete?

Although Linux binds 7F for BACKSPACE by historical issue,
*ced* regards 7F as DELETE instead of BACKSPACE.

If you have a problem with BACKSPACE and DELETE,
you should change your terminal-emulator's configuration.
