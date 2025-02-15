run:
    cargo 3ds run --address $N3DS_IP_ADDR
# currently doesn't use --server flag as it doesn't seem to work on wsl
# use `cp` command and run 3dslink on Windows instead

example EXAMPLE:
    cargo 3ds run --address $N3DS_IP_ADDR --example {{EXAMPLE}}


build:
    cargo 3ds build

debug:
    RUST_GDB="arm-none-eabi-gdb" rust-gdb target/armv6k-nintendo-3ds/debug/bevy_3ds.elf -ex "target remote $N3DS_IP_ADDR:4003"

macro-backtrace:
    RUSTFLAGS="-Z macro-backtrace" cargo 3ds build

cp:
    cargo 3ds build
    cp target/armv6k-nintendo-3ds/debug/bevy_3ds.3dsx /mnt/c/Users/$USER/3dsx/

cp-example EXAMPLE:
    cargo 3ds build --example {{EXAMPLE}}
    cp target/armv6k-nintendo-3ds/debug/examples/{{EXAMPLE}}.3dsx /mnt/c/Users/$USER/3dsx/
