# cargo-workspace

## Installation:

`cargo install cargo-workspace`

## Usage:

`cargo workspace COMMANDS`


## Examples:

```
// Executes check in all crates inside the workspace.
cargo workspace check

cargo workspace check --color=always

// Formates all crates inside the workspace.
cargo workspace fmt

// Builds all crates inside the workspace if a file as been changed.
cargo watch -x "workspace build"

```
