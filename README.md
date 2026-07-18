# Pinora CLI

Pinora CLI is a Rust-based scaffolding and project-management tool for creating ESP32 applications with a Rust firmware backend and a Tauri + React desktop interface.

It automates the repetitive setup work around generating an ESP-IDF firmware crate, creating a Tauri UI, installing dependencies, applying Pinora templates, recording project configuration, building firmware, flashing devices, and installing reusable hardware components.

> **Status:** Pinora CLI is in active early development. The current release is `0.1.0`, and commands, generated files, and project configuration may still change.

## What Pinora Creates

A new project is generated with two main folders:

```text
<project-name>/
├── Firmware/   # Rust ESP-IDF firmware
├── UI/         # Tauri + React + TypeScript desktop app
└── .espConfig/
    └── esp_config.json
```

Pinora also stores a lightweight list of known projects in the user's home directory:

```text
~/esp_rust_projects.json
```

## Current Features

- Create a complete Rust ESP32 + Tauri project
- Generate firmware from the official `esp-rs/esp-idf-template`
- Generate a React + TypeScript Tauri application with Bun
- Download and apply Pinora firmware and UI template files
- Install required Rust, frontend, and shadcn dependencies
- Save project paths, IDs, build commands, and installed components
- Build firmware from the project root
- Detect available serial ports
- Build, flash, and open the ESP32 monitor
- List hardware components from the remote component registry
- Download and register components in an existing firmware project
- Emit structured progress information for long-running operations

## Commands

### Create a project

```bash
pinora create <name>
```

Create the project in a specific existing directory:

```bash
pinora create <name> --path /path/to/parent-directory
```

Example:

```bash
pinora create lidar_controller --path ~/Desktop/Projects
```

Project names currently:

- must be between 3 and 100 characters
- cannot contain spaces
- cannot contain dots
- cannot contain hyphens
- cannot contain `/` or `\\`

Underscores are supported.

### Build firmware

Run this from the generated project directory or one of its child directories:

```bash
pinora build
```

Pinora locates `.espConfig/esp_config.json`, enters the firmware directory, and runs the build command saved in the project configuration.

### Build and flash

```bash
pinora run
```

When no port is supplied, Pinora displays the available serial ports and asks you to select one.

To select a port directly:

```bash
pinora run --port /dev/cu.usbserial-0001
```

The current implementation builds the firmware first and then flashes it with `espflash flash --monitor`.

### List available components

```bash
pinora listcomponents
```

This retrieves the currently available Rust hardware modules from the Pinora component registry.

### Install a component

```bash
pinora add <component>
```

Example:

```bash
pinora add ledmodule
```

The `.rs` extension is optional:

```bash
pinora add ledmodule.rs
```

Pinora downloads the component into:

```text
Firmware/src/module/
```

It then updates `src/module/mod.rs` and records the component in the project configuration.

### Show help

```bash
pinora help
```

## Requirements

Pinora currently expects the following tools to be installed and available through `PATH`:

- Rust and Cargo
- ESP-IDF Rust development environment
- `cargo-generate`
- `cargo-edit` for the `cargo add` command
- `espflash`
- Bun
- Git
- Tauri system prerequisites for your operating system

The generated firmware configuration currently uses:

```bash
source ~/export-esp.sh && cargo build
```

Your ESP-IDF export script must therefore exist at `~/export-esp.sh`, or the generated `.espConfig/esp_config.json` must be adjusted to use the correct command for your environment.

## Installation

Clone the repository and enter the project folder:

```bash
git clone <pinora-cli-repository-url>
cd pinora-cli
```

Install the CLI through Cargo:

```bash
cargo install --path .
```

Reinstall after making local changes:

```bash
cargo install --path . --force
```

Confirm that the command is available:

```bash
pinora help
```

Cargo normally installs the executable into:

```text
~/.cargo/bin/pinora
```

Make sure `~/.cargo/bin` is included in your `PATH`.

## Development

Build without installing:

```bash
cargo build
```

Run directly through Cargo:

```bash
cargo run -- help
```

Run a command during development:

```bash
cargo run -- create test_project --path ~/Desktop
```

### Just recipes

The repository includes a `justfile` with common development commands.

```bash
just build
```

```bash
just install
```

```bash
just release
```

The `release` recipe builds in release mode and force-installs the latest local binary.

## Generated Project Configuration

Each generated project contains:

```text
.espConfig/esp_config.json
```

The configuration currently stores information such as:

- project name
- unique project ID
- firmware path
- UI path
- firmware build command
- flash command
- installed components

Pinora searches upward from the current directory for this configuration, allowing project commands to work from the project root and nested folders.

## Component Registry

The current component registry is backed by the `src/module` directory in the Pinora firmware template repository on GitHub.

When a component is installed, Pinora:

1. loads the local project configuration
2. checks whether the component is already installed
3. queries the remote registry
4. downloads the selected Rust file
5. regenerates `Firmware/src/module/mod.rs`
6. updates the project configuration and local project database

A network connection is required for project template downloads and component installation.

## Technology

Pinora CLI is written in Rust and currently uses:

- Tokio for asynchronous execution
- Reqwest for GitHub and template downloads
- Serde and Serde JSON for project configuration
- Serialport for ESP32 port discovery
- Anyhow for error handling
- UUIDs for unique project identification

The generated projects combine:

- Rust
- ESP-IDF
- Tauri 2
- React
- TypeScript
- Vite
- Bun
- Tailwind CSS
- shadcn

## Project Structure

```text
src/
├── main.rs                       # Command parsing and command dispatch
├── commands.rs                   # Command module exports
├── commands/
│   ├── create.rs                 # Firmware and UI project generation
│   └── build.rs                  # Firmware build execution
├── progress.rs                   # Structured task progress reporting
├── project_config.rs             # Per-project configuration handling
├── project_config_database.rs    # Global project database handling
├── sharedtypes.rs                # Shared types, dependency lists, and constants
├── utility.rs                    # Downloads, dependencies, logging, and ports
└── file_json.rs                  # Reserved template-file loading work
```

## Known Limitations

- The command parser is currently custom rather than based on a dedicated CLI framework.
- Some user-facing error messages still reference the former `esp` command name and are being migrated to `pinora`.
- The generated build command currently assumes a Unix-like shell and `~/export-esp.sh`.
- The firmware output path is currently specific to the `xtensa-esp32-espidf` debug target.
- Project names cannot currently contain hyphens or dots.
- Template and component sources are currently tied to specific GitHub repository paths and branches.
- The project database does not yet expose commands for removing, renaming, or repairing registered projects.
- Partial project creation may leave generated folders behind when a later setup step fails.

## Roadmap

Planned areas of improvement include:

- complete migration of old command and repository references
- stronger argument parsing and validation
- configurable ESP-IDF environment setup
- support for additional ESP32 targets
- project listing and management commands
- safer recovery from partially completed project creation
- improved component metadata and compatibility information
- tighter integration with Pinora Studio
- automated release binaries and installers

## Related Projects

- **Pinora Studio** — the desktop interface for managing and interacting with Pinora projects
- **Pinora Template** — the source templates and firmware components used during project generation

## Contributing

Pinora is still evolving quickly. Before opening a pull request:

1. build the CLI with `cargo build`
2. test installation with `cargo install --path . --force`
3. create a fresh test project
4. verify firmware and UI generation
5. run the relevant build, component, and flashing commands

Keep changes focused and document any modification to generated project structure or configuration.

## License

No license has been declared yet. Until a license is added, the repository remains under standard copyright protection and reuse is not automatically granted.
