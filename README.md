# rnrm

**_manage npm registries base rust 🦀_**

# Benchmark Results: rnrm vs nrm

### Command Performance Comparison

| Command  | nrm (Min, Mean, Max)            | rnrm (Min, Mean, Max)           | Performance Change (%)   |
| -------- | ------------------------------- | ------------------------------- | ------------------------ |
| `ls`     | 106.27 ms, 108.57 ms, 110.87 ms | 136.21 µs, 137.92 µs, 139.85 µs | -99.87% (rnrm is faster) |
| `use`    | 102.87 ms, 105.46 ms, 108.01 ms | 1.9379 ms, 2.0982 ms, 2.2669 ms | -98.02% (rnrm is faster) |
| `add`    | 106.83 ms, 108.76 ms, 110.57 ms | 2.4322 ms, 2.6138 ms, 2.7835 ms | -97.60% (rnrm is faster) |
| `remove` | 95.288 ms, 97.447 ms, 99.657 ms | 369.44 µs, 379.37 µs, 390.45 µs | -99.61% (rnrm is faster) |

Therefore, we should use `rnrm`!!!
