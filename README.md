[![Rust](https://github.com/Hukadan/pobsdlib/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Hukadan/pobsdlib/actions/workflows/rust.yml)

### pobsdlib
Library in Rust to read the PlayOnBSD database 
(https://github.com/playonbsd/OpenBSD-Games-Database.)


### How to use the binary
You first need to compile the library and the binary.
```
$ cargo build --release
```

In the`target/release/` folder, you should find the binary `database2json`.

You can execute it with a database as argument:
```
$ ./database2json /path/to/your/database.db
```
