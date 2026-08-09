# Pinora CLI

Pinora CLI is a Rust-based project generator and management tool for building complete ESP32 systems with Rust firmware and a dedicated desktop interface.

Pinora is self-contained. It does not depend on Starboard or on a separate scaffolding application. The CLI creates the firmware, desktop UI, project metadata, and shared project commands that make up a Pinora project.

The firmware started from the official `esp-rs/esp-idf-template`, but has since evolved into Pinora's own firmware architecture. The desktop UI is likewise a heavily customized Electrobun application built with React and TypeScript rather than a stock starter or an external companion app.

> **Status:** Pinora CLI is in active early development. The current release is `0.1.0`, and commands, generated files, and project configuration may still change.

## What Pinora Creates

A new project is generated with two main folders:

```text
<project-name>/
├── Firmware/     # Pinora Rust + ESP-IDF firmware
├── UI/           # Pinora Electrobun + React + TypeScript desktop app
├── justfile      # Shared firmware and UI workflows
└── pinora.toml   # Project metadata
```

Pinora also stores a lightweight list of known projects in the user's home directory:

```text
~/esp_rust_projects.json
```

## Current Features

- Create a complete Pinora ESP32 project without an external scaffolding system
- Generate Pinora firmware derived from the official `esp-rs/esp-idf-template`
- Generate Pinora's custom Electrobun, React, and TypeScript desktop UI
- Install the root project commands and metadata shared by the firmware and UI
- Download the maintained Pinora firmware and UI source files
- Save project paths, IDs, build commands, and installed components
- Build firmware from the project root
- Detect available serial ports
- Build, flash, and open the ESP32 monitor
- List hardware components from the remote component registry
- Download and register components in an existing firmware project
- Emit structured progress information for long-running operations

## Firmware Architecture

Pinora firmware is built on Rust and ESP-IDF. Its original foundation came from the official `esp-rs/esp-idf-template`, which provides the ESP-IDF Rust toolchain and build integration. Pinora now layers its own architecture on top of that foundation, including:

- hardware and module abstractions
- built-in modules for common sensors and actuators
- command, event, registration, and shared protocol types
- Pinora-specific logging and utility code
- project-level build and flash workflows

The result should be treated as Pinora firmware, not as an unchanged copy of the upstream ESP-Rust template. The upstream template is the technical starting point; Pinora owns the generated structure and behavior.

## Desktop UI Architecture

The `UI/` directory contains Pinora's desktop application. It uses Electrobun with Bun, React, TypeScript, and Vite, but the application itself is a custom Pinora system rather than a stock Electrobun starter.

The UI includes Pinora-specific module definitions and views, runtime module state, serial communication workers, shared protocol types, logs, layout components, and device controls. It is generated as part of each project and communicates directly with the Pinora firmware architecture. No Starboard application or service is required.

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

Pinora resolves the current Pinora project, enters its firmware directory, and runs the project's configured build workflow.

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
- `espflash`
- Bun
- Git
- `just`
- the native build prerequisites required by Electrobun for your operating system

Generated projects include the ESP-Rust toolchain, Cargo target configuration, ESP-IDF defaults, and shared `just` recipes needed by the Pinora firmware workflow. Your ESP-IDF Rust environment must still be installed and available to those commands.

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

## Project Metadata

Each generated project contains:

```text
pinora.toml
```

This file identifies the project and keeps its root-level Pinora metadata with the firmware and UI it belongs to. The CLI also maintains a local project database containing operational information such as:

- project name
- unique project ID
- firmware path
- UI path
- firmware build command
- flash command
- installed components

This metadata is owned by Pinora and does not require Starboard or another project-management application.

## Component Registry

The current component registry is backed by the `src/module` directory in the Pinora firmware template repository on GitHub.

When a component is installed, Pinora:

1. loads the local project configuration
2. checks whether the component is already installed
3. queries the remote registry
4. downloads the selected Rust file
5. updates `Firmware/src/module/mod.rs`
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
- Electrobun
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
│   ├── build.rs                  # Firmware build execution
│   └── root/mod.rs               # Root project files and metadata
├── firmware/                     # Pinora firmware source manifest
├── ui/                           # Pinora desktop UI source manifest
├── module.rs                     # Component installation
├── progress.rs                   # Structured task progress reporting
├── project_config.rs             # Per-project configuration handling
├── project_config_database.rs    # Global project database handling
├── global_definition.rs          # Shared project and template types
└── utility.rs                    # Downloads, file generation, logging, and ports
```

## Known Limitations

- The command parser is currently custom rather than based on a dedicated CLI framework.
- Some user-facing error messages still reference the former `esp` command name and are being migrated to `pinora`.
- Configured build commands are currently executed through a Unix-like `bash` shell.
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
- automated release binaries and installers

## Project Sources

- **ESP-Rust / ESP-IDF template** — the upstream foundation from which Pinora firmware originally evolved
- **Pinora Template** — the Pinora-owned firmware, UI, root project files, and components downloaded during project generation

Pinora does not require Starboard or a separate desktop management system. The generated firmware and UI are the Pinora system.

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
