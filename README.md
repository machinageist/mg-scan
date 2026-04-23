# mg-scan
A port scanner built-in Rust. 
A learning in public project. 
Constructive criticism welcomed.

---

## Overview
This project mainly to help expand experience in Rust while gaining a useful tool for cyber security in the process.

---

## Tech Stack


---

## Project Structure

```
mg-scan/
├── Cargo.lock          # Dependencies and metadata
├── Cargo.toml          # Pinned dependency versions
├── README.md           # This file
└── src                 # 
    └── main.rs         # Entry point, and scanner logic temporarily
```
---

## Compiling

On Linux, navigate to desired directory and:
~~~
git clone https://github.com/machinageist/mg-scan/
cd mg-scan/
cargo build --release
~~~

---

## Installing 

On Linux:
~~~
cd /path/to/mg-scan
cp target/release/mg-scan /usr/local/bin
~~~

___

## Usage

~~~
mg-scan <domain>
~~~

---

## License

This project is licensed under the MIT License — see the LICENSE file for details.

---
