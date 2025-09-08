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
    |   ├── clear.rs
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

## ls (Rust Implementation)
```
This project is a Rust implementation of the Unix ls command.
It supports:

Basic listing of files and directories

-a → show hidden files (including . and ..)

-l → long listing format with permissions, owners, groups, sizes, and timestamps

-F → append indicators to entries (/ for directories, * for executables, @ for symlinks, etc.)

Column-based layout for short format (adjusts to terminal width)

Correct handling of symlinks (shows targets in -l mode)

Expansion of ~ to the user’s home directory

Handling of filenames that require quoting (spaces, special characters)

The behavior closely mimics GNU ls, including support for special cases like:

ls with no arguments → lists the current directory

ls - → tries to access a file literally named -

Block size reporting in long format (total shown in 1K blocks like GNU ls)
```
## echo (Rust Implementation)

Prints its arguments to standard output. It replicates the behavior of the Unix echo command, supporting escape sequences and output formatting.

## cd (Rust Implementation)

Changes the current working directory of the shell session. Supports relative and absolute paths, as well as shortcuts like ~ for the home directory and - to switch to the previous directory.

## clear (Rust Implementation)

Clears the terminal screen by issuing the appropriate escape sequences, giving the user a clean workspace. It behaves like the standard Unix clear command.

## pwd (Rust Implementation)

Prints the current working directory.

## cat (Rust Implementation)

Concatenates and displays the contents of files to standard output. Supports reading multiple files sequentially.

## cp (Rust Implementation)

Copies files and directories. Supports basic copying of files with preservation of file metadata where applicable.

## rm (Rust Implementation)

Removes files or directories. Implements safe deletion with handling of files and symbolic links.

## mv (Rust Implementation)

Moves or renames files and directories.

## mkdir (Rust Implementation)

Creates directories. Supports creation of nested directories.