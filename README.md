# rusternel

**rusternel** is a terminal user interface (TUI) application written in Rust that displays detailed system information, process lists, CPU and memory statistics, and more. The goal of this project is to provide a fast, modern, and extensible system monitor for Unix-like systems, inspired by tools like `htop` and `glances`.

## Features

- Display kernel and OS information
- Show system uptime and load averages
- List running processes with filtering capabilities
- (Planned) Visualize CPU and memory usage
- (Planned) Send signals to processes
- (Planned) Interactive navigation and search

## Getting Started

### Prerequisites

- Rust (edition 2021 or newer)
- Unix-like operating system (Linux recommended, as `/proc` is used)

### Building

```sh
cargo build --release
```
