# task

Simple Rust CLI program for lab variant 2: check whether a number is prime.

## Build and run

Use `mingw32-make` on Windows:

```powershell
cd "C:\Users\андрей\Desktop\учеба\Технологии распространения\task"
mingw32-make -f Makefile build
mingw32-make -f Makefile run
```

Or use Cargo directly:

```bash
cargo build
cargo run
cargo test
```

## Deb package

On Linux/WSL with `cargo-deb` installed:

```bash
cargo install cargo-deb
cargo deb
```

Output package path:

`target/debian/task_0.1.0_amd64.deb`
