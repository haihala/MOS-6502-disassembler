# Tools Programmer Homework

Disassembler for the MOS 6502 microprocessor family. Used
https://www.masswerk.at/6502/6502_instruction_set.html as a reference.

Used [cargo-make](https://github.com/sagiegurari/cargo-make) to manage tasks.
The project has the following tasks available:

- server: Starts the web server
- cli <files>: Runs the cli disassembler on all of the files provided
- watch: Runs server, rebuilds and restarts when files change
- clean: Runs cargo clean
- build: Runs cargo build
- test: Runs cargo test, server must be running
- format: Check formatting with rustfmt
- clippy: Check code with clippy
- validate: Runs build, test, format, and clippy tasks
- build-release: Builds the release binary
- build-docker: Builds a docker container and tags it as `haihala/mos-6502-disassembler`
- bench: Runs cargo bench, will take several minutes
- flamegraph-bench: Runs generates a flamegraph of the benchmarks with cargo-flamegraph
- flamegraph-cli: Runs generates a flamegraph of the cli with cargo-flamegraph

Run any of them with `cargo make <task>`. You need to install
[cargo-make](https://github.com/sagiegurari/cargo-make?tab=readme-ov-file#installation)
and if you want to generate flamegraphs,
[cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph?tab=readme-ov-file#installation).

When the server is running, you can find documentation at `/swagger`. If you
started the server using `cargo make server` or `cargo make watch`, the swagger
UI will be at http://127.0.0.1:9999/swagger. There is also a rudimentary
frontend available at http://127.0.0.1:9999/.

# Python testing tools

To help with testing, I made some simple python scripts.

## generate-binary.py

This script was used to generate the "mega binary". The mega binary is a binary
with one of each opcode and the operands set to 'FF' for each one. This was then
fed to the sample implementation at
https://www.masswerk.at/6502/disassembler.html to generate a test case, same as
other sample binaries that came with the repo. The main reason is that it tests
the massive match expression that maps the opcodes to operations and address
modes. A few mistakes were made when that table was being implemented which were
caught with the megabinary test. The opcodes were derived directly from the
reference page HTML.

If an additional "giga" argument is provided, instead of generating all of the
opcodes with FF for all operands, it will generate a binary with all of the
opcodes and all potential values for operands. Effectively this is 256 the size
of the mega binary. I am aware that the giga and mega terms are inaccurate. Felt
they were descriptive and slightly amusing.

There is a slight difference between the sample implementation and this one. If
fed the gigabinary, the sample implementation stops reading instructions after
the offset gets to 2^16. I thought it be better to read them and then present
the offset with additional bytes to make it fit.

## feed.py

The feed script can be used to feed binary files to the web server. To use it,
you need to install [httpx](https://www.python-httpx.org/). I recommend doing
this in a virtual python environent with something like:

```sh
python -m venv venv
./venv/bin/activate # This depends on shell and whatnot
pip install httpx
```

To use the feed script, have the first command line argument be the path to the
file you want to send over, for example `python feed.py test-bin/test1.bin`.
Optionally, you can add a `--verbose` flag after the path to get a bit more
information. This was used to test among other things that I understood how the
bytes are ordered. Like before, https://www.masswerk.at/6502/disassembler.html
was used as a reference implementation.

# CI

The project has a CI pipeline using Github Actions. The pipeline has the
following steps:

- Build a docker image using `cargo make build-docker`
- Start docker image
- Run checks with `cargo make validate` (tests need a running backend)
- Logs into docker hub and publishes the image

If a step fails, the execustion stops.

There is a server available at https://mos-6502-disassembler.tunk.org/ that
pulls the latest image every night and runs that behind an nginx container.
Certificates are handled using certbot / letsencrypt and the domain comes from
dy.fi. VM itself is from the Oracle cloud free tier. If I could run the
container directly I would but Oracle free tier doesn't let me. The server is
quite puny and won't handle heavy traffic. Considering how hosting it was not in
the assignment, I will assume this is fine.

# Performance

I did some benchmarking and profiling using `criterion` and `cargo-flamegraph`
respectively. Before I changed anything, disassembling the mega binary took
about 5ms. After some tweaks I got it down to just above 4ms, but felt the loss
in readability was not worth the performance gain considering it should be
plenty fast for a web server as is.

The main culprit for performance issues is formating and concatenating strings.
Disassembling the gigabinary takes about 280ms (220ms on release build). If
output is piped to /dev/null, it drops to 90ms (40ms on release build). This is
increased when running the benchmarks, as those use `std::hint::black_box`. The
mos-6502 shouldn't even be able to handle a binary of that size, as the offset
won't fit to 16 bits.

I tried rayon for parallilizing the string operations and it did yield promising
results, but only beyond a certain point. The giga binary benchmark went down
about 50%. Unfortunately the mega binary went up about 50%.

# Levels

Despite the pretty lenient deadline I set for myself, I think it may be
beneficial to provide several levels of effort. Sometimes a tool needs to be
done yesterday and it only needs to be used once. Sometimes it needs to provide
the functionality described with a bit of sugar on top. Sometimes the user
requires something they can rely on for the rest of their career.

Levels 1 and 2 have corresponding tags in git and a brief description of the
philosophy behind it below. Level 3 is the final submission.

## Level 1

[link to tag](https://github.com/haihala/MOS-6502-disassembler/releases/tag/level-1)

This took me about 2h and basically just barely handles the example in the
original README.md. If it needed to be done as quickly as possible and had a
limited requirement set, I would go with an approach like this. Performance,
readability, and maintainability are not problems when something only needs to
run once and can be rewritten in a day. Under normal conditions I would probably
write this as a python script. I find python marginally faster to write for
basic scripting like this.

All of the code is in a single file. I didn't add any tests. No additional
checks of any kind besides what my editor was already set to do. No CI pipeline.
Simply put, it aims to solve the smallest amount of problems while still doing
what the task requires. I even kept the original README.md.

Unless explicitly prompted, I would **not** take this many shortcuts at work.
This is just to demonstrate what I can get done in 2h with no prior knowledge of
the domain. This is the first disassembler I ever wrote and I tend to not work
this close to metal in general.

## Level 2

[link to tag](https://github.com/haihala/MOS-6502-disassembler/releases/tag/level-2)

The second level is a more reasonable one. This took me about a work day to put
together (including level 1). Level 2 is about completeness and the low hanging
tooling fruit. I added cargo-make as a task runner. With that, I set up basic
validation using clippy and rustfmt. The biggest gain from cargo-make are the
automatic restarts with the watch feature and the ability to run `cargo make
validate` to check if it builds, tests pass, formatting is up to snuff, and
clippy is happy. I could use bacon, but didn't think it necessary. I considered
adding a GitHub actions CI pipeline, but I have ideas for level 3 so I kicked
that can down the road.

I added tests for both of the sample binaries. The expected output was from the
reference implmemetation.

I made [#feed.py]. This was somewhat unnecessary in hindsight, but I wanted to
be sure the reference website was reading the bytes in the same order. It helped
me with some early debugging when I was checking for differences between my
output and the one provided by masswerk with `diff`. I'm considering deleting it
now that it serves no purpose.

The code was split up to multiple files to separate disassembly logic from the
web server presentation logic. This piqued my curiosity and I wanted to see if I
could make a cli application, as that may be useful for some and after 30 or so
lines of rust I had one. This makes [#feed.py] irrelevant.

I implemented the entirety of the instruction set and both of the test binaries
disassemble correctly. I wrote a big match statement to translate opcodes to
operations and address modes, which I also made into separate enums. The massive
match may not be the prettiest thing in the world, but I like the explicitness
of it. If instead of that, it looked up the instructions in some (likely csv)
file derived from the HTML of masswerk, it would be slightly harder to see what
maps to what. The compiler would also likely be less able to optimize, but
that's a baseless hunch. I did consider moving the match somewhere, but the file
is not _that_ long so I didn't. Generally I try to aim for less than 300 lines
per file, but I liked the logical consistency of everything related to the
disassembly being in the same file. Navigating it is somewhat difficult if you
don't have the proper tooling installed, but I don't think one should structure
their code with that in mind. Navigating with rust-analyzer scales much better.

## Level 3

No tag, see latest main branch.

I got most of this done by early Saturday of week 1, meaning I still had over a
week left before the deadline. In terms of work hours it was probably around 3
full days. I spent random bursts of of time fine tuning after that, but none of
the changes were that significant. The themes of level 3 were correctness and
expanding usability.

The most notable addition was the frontend, where one could use the system
through a web browser. I used [Askama](https://crates.io/crates/askama)
templates on the backend and [HTMX](https://htmx.org/) in the browser to add
some interactivity without having to do a full page load. I'm satisfied with the
simplicity of it all. HTMX has been something I've been circling for a while and
this gave me an opportunity to try it out. In about three lines of html, I got
all the interactivity I wanted. You can upload files and have the bytes decoded
into hexadecimal text representation and have your hexadecimal text disassembled
to a table of instructions. Askama had a few positive surprises. It uses rust
macros to check the template variables at compile time. A missing template
variable won't compile and an additional one is a warning. The templates get
baked into the binary, so packaging the application was simple. I'll likely use
both HTMX and Askama again.

I wanted an OpenAPI/Swagger documentation and initially used a crate called
utoipa to generate the OpenAPI spec. While writing the frontend I realized it
wasn't quite as ergonomic as I had hoped. I probably spent more time with utoipa
than it would've taken to write the OpenAPI spec by hand, but I didn't accept
this as an option. Utoipa had subpar documentation and error messages. It was
missing some compile time safety that I've come to expect from rust. The code
compiled just fine even if the spec was incomplete, giving a runtime warning if
you open the swagger page. After trying a few things, I switched axum and utoipa
for poem, which comes with a first party OpenAPI generator. I liked how poem
structured the routes and how little additional work the api docs required.
While it wasn't perfect, I will probably end up using poem again as well. My one
gripe comes with the testing module, which doesn't allow asserting if a text
response contains text, just that it equals a text and I didn't want to have my
frontend tests break on indentation changes nor did I want to mix poem tests and
reqwest, so I just stuck with reqwest.

Finally on the frontend/presentation side, I set the whole thing up in oracle
cloud through docker. The details are explained [here](#CI). The whole CI
pipeline was implemented for level 3.

On the correctness side, I was most concerned about the big match statement
which decodes opcodes. I was almost certain there was a mistake in there that
wasn't caught by the test binaries. To check, I scraped the HTML table from the
reference site, parsed it into a csv file and used that and made the python
script [#generate-binary.py]. I then generated a test case from this similar to
how I had done for the test binaries included with the repo. Lo and behold, I
had made a few errors while transcribing the table. Fixed those and felt good
about myself. I also added tests for the frontend, which was surprisingly easy
because it's mostly template rendering.

I benchmarked the disassembly function and used cargo-flamegraph to see where
the time is going. You can read more about that [here](#Performance). I did a
couple of trivial optimizations here like removing unnecessary clones where I
can use borrows, but left most of the code unchanged.

Some of the level 3 tests are a good idea and in hindsight I think the optimal
amount of effort for this assignment would be somewhere between levels 2 and 3.
The frontend was not something I would do by default given the initial
assignment, I just felt like trying it out and since I have plenty of time I saw
no harm in this detour.
