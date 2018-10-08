# the-elements
temperature monitoring

## Build

`cargo +nightly-msvc build --release`

`cargo +nightly-msvc size -- -A target/thumbv7m-none-eabi/release/the-elements`

`cargo +nightly-msvc objdump -- -disassemble -no-show-raw-insn -print-imm-hex target/thumbv7m-none-eabi/release/the-elements`

## Run

`openocd`

`cargo +nightly-msvc run --release`
