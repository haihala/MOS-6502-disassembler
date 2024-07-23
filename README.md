# Tools Programmer Homework

Disassembler for the MOS 6502 microprocessor family. Used
https://www.masswerk.at/6502/6502_instruction_set.html as a reference.

Used https://github.com/sagiegurari/cargo-make to manage tasks. The project has
the following tasks available:

- clean: Runs cargo clean
- build: Runs cargo build
- test: Runs cargo test
- format: Check formatting with rustfmt
- clippy: Check code with clippy
- validate: Runs build, test, format, and clippy tasks

Run any of them with `cargo make <task>`. You need to install cargo-make.

# Python testing tool

To easier feed files to the web server, I made a simple python testing tool
called `feed.py`. To use it, you need to install
[httpx](https://www.python-httpx.org/). I recommend doing this in a virtual
python environent with something like:

```sh
python -m venv venv
./venv/bin/activate # This depends on shell and whatnot
pip install httpx
```

You can then use the feeder. To use it, have the first command line argument be
the path to the file, for example `python feed.py test-bin/test1.bin`.
Optionally, you can add a `--verbose` flag after the path to get a bit more
information.
