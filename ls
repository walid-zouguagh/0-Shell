# 0-Shell

#### 0-shell is a lightweight Unix-like shell written in Rust. It reimplements core Unix commands directly through system calls, without depending on external programs or traditional shells such as bash or sh.

## Project Layout

```
0-shell/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── shell.rs
    ├── parser.rs
    ├── commands/
    │   ├── mod.rs
    │   ├── echo.rs
    │   ├── cd.rs
    │   ├── ls.rs
    │   ├── pwd.rs
    │   ├── cat.rs
    │   ├── cp.rs
    │   ├── rm.rs
    │   ├── mv.rs
    │   └── mkdir.rs
    └── utils.rs
```

## main.rs
Entry point, very small : Initializes the REPL loop via Shell::run()

## shell.rs
The REPL loop (prompt, read line, call parser, dispatch command)
Handles Ctrl+D / Ctrl+C gracefully
Owns global state (like current directory, maybe history later)