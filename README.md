# Rust Simpe Bits - Asset Downloader

## Overview

`rsb-downloader` is a Rust library for downloading assets from one (as fallback from multiple) source to a dotfile of the home directory.

## Features

- Simple API for downloading files
- Synchronous and asynchronous support


## Installation

Add the following to your `Cargo.toml`:

### Sync version

```bash
cargo add rsb-downloader --features sync
```

### Async version

```bash
cargo add rsb-downloader --features async
```
This also requires tokio to work.

```bash
cargo add tokio --features full
```


## Usage

See Examples for the usage.
The examples can be executed in the following way:


### Sync Example

```bash
 cargo run --example sync --features sync
```

### Async Example

```bash
 cargo run --example async --features async
```
