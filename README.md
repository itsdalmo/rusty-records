## Rusty-records

If you records are rusty, you might want to clean them up before loading.

## Usage

```bash
cat ./examples/data/sample.txt |\
    rusty-records --mode mapper |\
    rusty-records --mode reducer
```
## Test

```bash
cat ./examples/data/sample.txt |\
    ./target/release/rusty-records --mode mapper |\
    ./target/release/rusty-records --mode reducer
```

