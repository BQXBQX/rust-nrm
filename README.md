# rust-nrm

A fast and efficient NPM registry manager written in Rust 🦀

## Features

- 🚀 Fast registry switching
- 📋 List available registries
- ➕ Add custom registries
- 🗑️ Remove registries
- 🔄 Switch between registries (global/local)
- ⚡ Test registry speeds

## Installation

### From Cargo

```bash
cargo install rust-nrm
```

### From Source

```bash
git clone https://github.com/BQXBQX/rnrm.git
cd rnrm
cargo install --path .
```

## Usage

```bash
# List all registries
rust-nrm ls

# Switch to a registry
rust-nrm use npm
rust-nrm use taobao

# Use registry for current project only
rust-nrm use npm --local

# Add a custom registry
rust-nrm add custom https://custom.registry.com/

# Remove a registry
rust-nrm rm custom

# Test registry speeds
rust-nrm test
```

## Available Registries

- npm - https://registry.npmjs.org/
- yarn - https://registry.yarnpkg.com/
- taobao - https://registry.npmmirror.com/
- tencent - https://mirrors.cloud.tencent.com/npm/
- npmMirror - https://skimdb.npmjs.com/registry/
- github - https://npm.pkg.github.com/

## Benchmark Results: rnrm vs nrm

### Command Performance Comparison

| Command  | nrm (Min, Mean, Max)            | rnrm (Min, Mean, Max)           | Performance Change (%)   |
| -------- | ------------------------------- | ------------------------------- | ------------------------ |
| `ls`     | 106.27 ms, 108.57 ms, 110.87 ms | 136.21 µs, 137.92 µs, 139.85 µs | -99.87% (rnrm is faster) |
| `use`    | 102.87 ms, 105.46 ms, 108.01 ms | 1.9379 ms, 2.0982 ms, 2.2669 ms | -98.02% (rnrm is faster) |
| `add`    | 106.83 ms, 108.76 ms, 110.57 ms | 2.4322 ms, 2.6138 ms, 2.7835 ms | -97.60% (rnrm is faster) |
| `remove` | 95.288 ms, 97.447 ms, 99.657 ms | 369.44 µs, 379.37 µs, 390.45 µs | -99.61% (rnrm is faster) |

### **Therefore, we should use `rnrm`!!!**

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
