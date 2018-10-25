# the-elements
temperature monitoring

## Build

`cargo build --release`

`cargo size -- -A target/thumbv7m-none-eabi/release/the-elements`

`cargo objdump -- -disassemble -no-show-raw-insn -print-imm-hex target/thumbv7m-none-eabi/release/the-elements`

## Run

`openocd &`

`cargo run --release`
