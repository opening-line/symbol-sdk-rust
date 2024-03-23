# How to test

``` bash
cargo test --release
cargo test --release --all-features
```

# Generate catbuffer (src/symbol/models.rs)

```bash
./scripts/run_catbuffer_generator.sh
```

# Generate Test (tests/symbol_models.rs)

``` bash
./scripts/run_testvectors_generator.sh
```

# Prerequisites

``` bash
# setup for Test.
$ sudo apt install curl
$ curl https://sh.rustup.rs -sSf | sh

# setup for Generator.
$ sudo apt install python3 python3-pip
$ pip3 install -r generator/requirements.txt
```