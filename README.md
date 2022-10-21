# Experiment with source level debug of std lib

To source debug standard libraries using `vscode` you need to setup "sourceMap" in launch.json
as mentioned in the [manual](https://github.com/vadimcn/vscode-lldb/blob/ed956cefe3911bd96e7bbbd5b847a137822d548c/MANUAL.md#source-path-remapping)
and more info [here](https://users.rust-lang.org/t/solved-how-to-step-into-std-source-code-when-debugging-in-vs-code/25319/5).

The goal here is to define the `sourceMap` "key" "value" pair.

But first we need to find the "Source location" by stepping into the assembly code of a
std::lib function. In this project, if you set a break point at line 4:
    `let mut cmd = Command::new(program);`
and then step into that function, using `F11` on the keyboard, you'll see
"id" and "Source location" lines at the top of the display:
```
  ; id = {0x00000387}, range = [0x000000000000d360-0x000000000000d3f5), name="std::process::Command::new", mangled="_ZN3std7process7Command3new17hf3fb9ac31271353aE"
  ; Source location: /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/srce/process.rs:557
```
You divide the "Source location" into three pieces, `rustc/{uuid}`, `file-path` and `line-number`.
The `rustc/{uuid}` is "/rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52" the `file-path`
is "library/std/src/process.rs", `line-number` is 557. Another important piece is
the `name` of the function which is "name=" in the "id" line and is "std::process::Command::new".

The "key" for the `sourceMap` is `rustc/{uuid}` or "/rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52".
The "value" for the `sourceMap` is the "sysroot" plus `local-dir`.
The "sysroot" is found using `rustc --print sysroot`:
```
  $ rustc --print sysroot
  /home/wink/.rustup/toolchains/stable-x86_64-unknown-linux-gnu
```

The "value" can be found using linux `find` in the sysroot/ and `grep` those
found files for the `file-path`:
```
$ find /home/wink/.rustup/toolchains/stable-x86_64-unknown-linux-gnu -type f -name process.rs | grep 'library/std/src/process.rs'
/home/wink/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/process.rs
```

To create the "value" use the first part of the `full-path` for the file, but without the `file-path`
in this case `full-path` is:
```
"/home/wink/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/process.rs".
```

At this point use the `full-path` and validate this is the correct file by validating at
`line 557` of  "/home/wink/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/process.rs"
we see `pub fn new` being defined:
```
  pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
  	Command { inner: imp::Command::new(program.as_ref()) }
  }
```
If this is **NOT** the case "value" you most likely have the wrong "sysroot", and you
might just use the `find` command to find all `process.rs` and the look at each for
the expected function, in this case `pub fn new`.

Assuming all is well, we can now create the "value" from what's left after removing the `file-path` from the `full-path`:
```
"/home/wink/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust"
```

So the "sourceMap" in this case is:
```
"sourceMap": { "/rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52": "/home/wink/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust"},
```

You can now restart the debugging session and use "Command pallet" (Ctrl+Shift+P) to execute "toggle disassembly".
I've assigned "toggle disassembly" to (Ctrl+Shift+Alt+D), see
[keybindings in vscode manual](https://code.visualstudio.com/docs/getstarted/keybindings) or do a
[search](https://www.google.com/search?q=add+keyboard+shortcut+in+vscode).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

