## Rusty-records

If you records are rusty, you might want to clean them up before loading.

## Usage

```bash
cat ./examples/data/sample.txt |\
    mapper |\
    reducer
```

You can also specify input and output filenames (defaults to stdin/out):
```bash
mapper --input ./examples/data/sample.txt | reducer --output test.txt
```

## Example data

```bash
cat ./examples/data/sample.txt |\
    ./target/release/mapper |\
    ./target/release/reducer
```

## Throughput

```bash
yes |\
  ./target/release/mapper |\
  ./target/release/reducer |\
  pv > /dev/null
```

