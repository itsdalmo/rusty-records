## Rusty-records

If you records are rusty, you might want to clean them up before loading.

## Usage

```bash
cat ./examples/data/sample.txt |\
    mapper |\
    reducer
```
## Test

```bash
cat ./examples/data/sample.txt |\
    ./target/release/mapper |\
    ./target/release/reducer
```

