#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT_DIR="$SCRIPT_DIR/bin"
mkdir -p "$OUT_DIR"

echo "[+] Building AArch64 test binary fixtures..."

# 1. Compile Assembly Fixtures (*.s)
if [ -d "$SCRIPT_DIR/fixtures/asm" ]; then
    for src in "$SCRIPT_DIR/fixtures/asm"/*.s; do
        if [ -f "$src" ]; then
            name=$(basename "$src" .s)
            out_bin="$OUT_DIR/${name}"
            if command -v aarch64-linux-gnu-gcc &> /dev/null; then
                aarch64-linux-gnu-gcc -nostdlib -static "$src" -o "$out_bin"
                echo "  [ASM] Compiled $name -> tests/bin/${name}"
            fi
        fi
    done
fi

# 2. Compile C Fixtures (*.c)
if [ -d "$SCRIPT_DIR/fixtures/c" ]; then
    for src in "$SCRIPT_DIR/fixtures/c"/*.c; do
        if [ -f "$src" ]; then
            name=$(basename "$src" .c)
            out_bin="$OUT_DIR/${name}"
            if command -v aarch64-linux-gnu-gcc &> /dev/null; then
                aarch64-linux-gnu-gcc -nostdlib -static "$src" -o "$out_bin"
                echo "  [C]   Compiled $name -> tests/bin/${name}"
            fi
        fi
    done
fi

# 3. Compile Rust Fixtures (*.rs)
if [ -d "$SCRIPT_DIR/fixtures/rust" ]; then
    for src in "$SCRIPT_DIR/fixtures/rust"/*.rs; do
        if [ -f "$src" ]; then
            name=$(basename "$src" .rs)
            out_bin="$OUT_DIR/${name}"
            rustc --target aarch64-unknown-linux-gnu -C linker=aarch64-linux-gnu-gcc -C relocation-model=static -C link-args="-nostdlib -static -Wl,-Ttext=0x400000" -C panic=abort "$src" -o "$out_bin"
            echo "  [Rust] Compiled $name -> tests/bin/${name}"
        fi
    done
fi

echo "[+] All test fixtures built successfully into tests/bin/"
