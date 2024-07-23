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
- watch: Runs server in cargo-watch
- cli <files>: Runs the disassembler on all of the files provided

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

# Levels

Despite the pretty lenient deadline I set for myself, I think it may be
beneficial to provide several levels of effort. Sometimes a tool needs to be
done yesterday and it only needs to be used once. Sometimes it needs to provide
the functionality described with a bit of sugar on top. Sometimes the user
requires something they can rely on for the rest of their career.

Each level has a corresponding tag in git and a brief description of the
philosophy behind it below.

## Level 1

[link](https://github.com/haihala/MOS-6502-disassembler/releases/tag/level-1)

This took me about 2h and basically just barely handles the example in the
original README.md. If it needed to be done as quickly as possible and had a
limited restricted requirement set, I would go with an approach like this.
Performance, readability, and maintainability are not problems when something
only needs to run once and can be rewritten in a day. Under normal conditions I
would probably write this as a python script as despite my proficiency in rust I
find python marginally faster to write for basic scripting like this.

All of the code is in a single file. I didn't add any tests besides what was
provided with the template. No additional linting of any kind besides what my
editor was already set to do. No CI pipeline. Simply put, it aims to solve the
smallest amount of problems while still doing what the task requires. I even
kept the original README.md.

Unless explicitly prompted, I would not take this many shortcuts at work. This
is just to demonstrate what I can get done in 2h with no prior knowledge of the
domain. This is the first disassembler I ever wrote and I tend to not work this
close to metal in general.

## Level 2

[link](https://github.com/haihala/MOS-6502-disassembler/releases/tag/level-2)

The second level is a more reasonable one. This took me about a work day of
effort to put together (including level 1). Level 2 is about completeness and
the low hanging tooling fruit. I added cargo-make as a task runner. I like how
it automatically installs tools and has plenty of handy features. With that, I
set up basic validation using clippy and rustfmt. The biggest gain from
cargo-make are the automatic restarts with the watch feature and the ability to
run `cargo make validate` to check if it builds, tests pass, formatting is up to
snuff , and clippy is happy. I could use bacon, but didn't think it necessary. I
considered adding a GitHub actions CI pipeline, but I have ideas for level 3 so
I kicked that can down the road.

During this level I heavily used https://www.masswerk.at/6502/disassembler.html
to validate my disassembly and at the end managed to have essentially identical
output for both of the provided test binaries.

I made a python script to feed the binaries to the web server. This was somewhat
unnecessary in hindsight, but I wanted to be sure the reference website was
reading the bytes in the same order and there was no misunderstanding. It helped
me with some early debugging when I was checking for differences between my
output and the one provided by masswerk with `diff`. Considering deleting it now
that it serves next to no purpose.

The code was split up to multiple files to separate disassembly logic from the
web server presentation logic. This piqued my curiosity and I wanted to see if I
could make a cli application, as that may be useful for some and after 30 or so
lines of rust I had one.

I implemented the entirety of the instruction set and both of the test binaries
disassemble. The way I did that wasn't necessarily the cleanest and I would like
to validate I got everything there right. I wrote a big match manually with
heavy use of vim macros. The massive match may not be the prettiest thing in the
world, but I like the explicitness of it. If instead of that, it looked up the
instructions in some (likely csv) file derived from the HTML of masswerk, it
would be slightly harder to see what maps to what. I did consider moving the
match somewhere, but the file is not _that_ long yet so I didn't. Generally I
try to aim for less than 300 lines, but I like the logical consistency of
everything in that file right now. This may change later.
