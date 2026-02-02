# judgers

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/joseph-beck/judgers/checks.yml?style=plastic&label=checks)
![GitHub Release](https://img.shields.io/github/v/release/joseph-beck/judgers?style=plastic)
![GitHub License](https://img.shields.io/github/license/joseph-beck/judgers?style=plastic)

## what does judgers do?

helps streamline judging of things. from generating a judging allocations to a complete judging solution in a spreadsheet.

## usage?

```sh
# 1. clone the repo
git clone https://github.com/joseph-beck/judgers.git

# 2. build the binary
cargo build --release --bin judgers-cli

# 3. rename the binary if you fancy, you can also move somewhere fancy if you want
mv target/release/judgers-cli judgers

# 4. use judgers
./judgers
```

alternatively install with cargo

```sh
# 1. install with cargo
cargo install --path judgers-cli

# 2. use judgers
judgers-cli
```

## examples

check out [example](/examples) configs here.

for example could run the command below to create a spreadsheet!

```sh
judgers spreadsheet allocation.config.json -c spreadsheet.config.json
```
