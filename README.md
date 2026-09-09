# 🧪 maarch64-tests: Integration Test Suite & Fixtures

`maarch64-tests` contains the comprehensive automated test suite and precompiled AArch64 test fixtures for Maarch64.

---

## 🏃 Running Tests

```bash
# Run all 39+ integration tests
cargo test -p maarch64-tests

# Run a specific integration test
cargo test -p maarch64-tests --test lua_test
cargo test -p maarch64-tests --test sqlite_test
cargo test -p maarch64-tests --test macho_test
cargo test -p maarch64-tests --test busybox_utility_test
```

---

## 📦 Test Suite Overview

| Test Module | Coverage & Verification Area |
| :--- | :--- |
| **`macho_test.rs`** | Mach-O 64-bit ARM64 loader, segment mapping, and Darwin ABI stack structure. |
| **`busybox_utility_test.rs`** | 18 BusyBox utilities (`echo`, `cat`, `uname`, `pwd`, `ls`, `env`, `whoami`, `id`, `head`, etc.) executed statically and dynamically. |
| **`lua_test.rs`** | Complete execution of Lua 5.4.6 bytecode interpreter on ARM64 under Maarch64. |
| **`sqlite_test.rs`** | SQLite 3.42.0 database engine execution with SIMD vector operations. |
| **`rust_std_test.rs`** | Real-world Rust standard library runtime (`std::env`, `println!`) execution. |
| **`rust_grep_test.rs`** | Real-world Rust CLI application executing regex searches on ARM64. |
| **`pthread_test.rs`** | Multi-threaded POSIX thread creation, condition variables, and mutex synchronization. |
| **`gpu_thunk_test.rs`** | OpenGL ES / EGL dynamic thunking and 60 FPS rendering pipeline. |
| **`gtk_test.rs`** / **`qt_test.rs`** / **`sdl2_test.rs`** | GUI toolkit library thunking and window lifecycle management. |
| **`diff_test.rs`** | Step-by-step CPU register differential execution against the Unicorn Engine oracle. |
| **`instruction_test.rs`** / **`jit_fp_test.rs`** | Low-level ARM64 instruction correctness (`ADRP`, `STP/LDP`, `CMP`, SIMD, `TPIDR_EL0`). |
| **`android_test.rs`** | Android NDK surface and asset manager thunking. |
