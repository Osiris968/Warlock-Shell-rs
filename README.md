# Warlock Shell

A small Unix shell written in Rust. Warlock reads a line, runs builtins itself, and runs everything else by forking and calling `execvp`. It also supports simple pipes, `&&` chains, aliases, and a colored prompt you can configure.

```
mkestner@macbook ~/Projects/Warlock-Shell-rs> ls -l | grep src
drwxr-xr-x@ 8 mkestner staff 256 Nov  5 18:45 src
```

## Features

- **Colored prompt**: shows `user@host path>`, with your home directory shortened to `~`
- **Builtins**: `cd`, `clear`, `type`, `help`, `exit`, `warlock_gen_config`
- **Pipes**: `cmd1 args | cmd2 args`
- **Chaining**: `cmd1 && cmd2` runs the second command only if the first one succeeds
- **Tilde expansion**: `~` and `~/path` expand to your home directory
- **Aliases** and **prompt color**, set in a config file
- **Ctrl+D** exits the shell

## Requirements

- Rust with **edition 2024** support (Rust 1.85 or newer)
- A Unix-like OS (Linux or macOS). Warlock uses `fork`/`execvp` through the [`nix`](https://crates.io/crates/nix) crate, so it won't build on Windows.

## Building and running

```sh
git clone git@github.com:Osiris968/Warlock-Shell-rs.git
cd Warlock-Shell-rs
cargo run --release
```

The binary is built from `src/bin/main.rs`, so it ends up at `target/release/main`.

## Builtin commands

| Command              | Description                                                       |
| -------------------- | ----------------------------------------------------------------- |
| `cd [dir]`           | Change directory. With no argument, goes to your home directory.  |
| `clear`              | Clear the screen.                                                 |
| `type <cmd>`         | Say whether `<cmd>` is a builtin or where it is on your `$PATH`.  |
| `help`               | List the supported builtins.                                      |
| `warlock_gen_config` | Create a starter config file (see below).                         |
| `exit`               | Quit the shell. Ctrl+D also works.                                |

## Configuration

Warlock reads its config from:

```
~/.config/warlock-shell/warlock.conf
```

To create a starter file, run `warlock_gen_config` inside the shell. It won't overwrite a config that already exists. If there's no config file, Warlock prints a notice on startup and uses the defaults.

### Format

Each line is a `key = value` pair. Lines that start with `#` are comments, and anything after a `#` on a line is ignored.

```ini
# Prompt color: red, green, yellow, blue, purple, cyan, white
# Default is green.
prompt_color = cyan

# Aliases: "alias <name> = <command>"
alias ll = ls -la
alias gs = git status
```

Config changes take effect the next time you start the shell.

## Project layout

```
src/
├── bin/main.rs              # REPL loop: prompt, read input, run commands
├── lib.rs                   # Command parsing, tilde expansion, prompt building
├── commands/builtins.rs     # Builtins plus fork/exec
└── configs/
    ├── config_reader.rs     # Config file creation and parsing
    └── shell_modules.rs     # Pipes, && chains, prompt colors, aliases
```

## Known limitations

Warlock is a work in progress. These things aren't supported yet:

- Quoting and escaping. Input is split on whitespace only, so `echo "a b"` gets the arguments `"a` and `b"`.
- More than one `|` or `&&` in a single line, and mixing pipes with chains
- Redirection (`>`, `<`, `>>`), background jobs (`&`), and `;`
- Environment variable expansion (`$VAR`)
- Aliases inside pipes or chains. Aliases only apply to the first word of a plain command, and any arguments typed after the alias are dropped.
- Command history and line editing

## License

Copyright (c) 2026 Michael Kestner. All Rights Reserved.
