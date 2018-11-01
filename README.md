# the-elements
temperature monitoring

## Build

`cargo build --release`

`cargo size --bin the-elements --release`

`cargo objdump --bin the-elements --release -- -disassemble -no-show-raw-insn -print-imm-hex`

## Run

`openocd &`

`cargo run --release`
