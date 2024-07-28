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
- build-release: Builds the release binary
- build-docker: Builds a docker container and tags it as `mos-6502-disassembler`
- bench: Runs cargo bench
- flamegraph: Runs generates a flamegraph of the benchmarks with cargo-flamegraph

Run any of them with `cargo make <task>`. You need to install cargo-make.

When the server is running, you can find documentation at `/swagger`. If you
started the server using `cargo make server` or `cargo make watch`, the swagger
UI will be at http://127.0.0.1:9999/swagger. There is also a rudimentary user
interface available http://127.0.0.1:9999/.

# Python testing tools

To ease testing, I made some simple python testing tools. These are not built in
with the web server, but simply used to generate test data and more easily
access the web server.

## feed.py

The feed script can be used to feed binary files to the web server. To use it,
you need to install [httpx](https://www.python-httpx.org/). I recommend doing
this in a virtual python environent with something like:

```sh
python -m venv venv
./venv/bin/activate # This depends on shell and whatnot
pip install httpx
```

You can then use the feeder. To use it, have the first command line argument be
the path to the file, for example `python feed.py test-bin/test1.bin`.
Optionally, you can add a `--verbose` flag after the path to get a bit more
information.

## generate-binary.py

This script was used to generate the "megabinary". Simply put, it is a binary
with one of each opcode and the operands set to 'FF' for each. This was then fed
to the sample implementation at https://www.masswerk.at/6502/disassembler.html
and the output compared to the web server's, same as other sample binaries. The
main advantage of this approach, is that it tests the massive match expression
that maps the opcodes to operations and address modes. A few mistakes were made
when that table was being implemented which were caught with the megabinary
test.

The opcodes were derived directly from the reference page HTML. This was edited
down to a csv variant with semicolons as separators and then fed to the python
script in question.

If an additional "giga" argument is provided, instead of generating all of the
opcodes with FF for all operands, it will generate a binary with all of the
opcodes and all potential values for operands. Not individual operands, each
loop of opcodes will all receive the same operands. The binary produced is 256
times the size of the giga binary.

There is a slight difference between the sample implementation and this one. If
fed the gigabinary, the sample implementation simply stops reading instructions
after the offset has capped out. I thought it be better to read them and then
present the offset with additional bytes to make it fit.

I am aware that the giga and mega terms are inaccurate. Felt they were
descriptive and slightly amusing.

# CI

The project has a CI pipeline using Github Actions. The pipeline not only
checks the code with all of the tools listed above, but builds a docker image
and pushes it to docker hub.

There is a server available at https://mos-6502-disassembler.tunk.org/ that then
every night pulls the latest image and runs that behind an nginx container.
Certificates are handled using certbot and the domain comes from dy.fi. VM
itself is from the Oracle cloud free tier.

The server is quite puny, so a large enough binary is likely to cause issues.
Considering how hosting it was not in the assignment I will assume this is fine.
If it was a concern I would put some work into configuring the nginx to limit
inputs, as that seems like a natural place to do that.

# Performance

I did some benchmarking and profiling using `criterion` and `cargo-flamegraph`
respectively. Before I changed anything, disassembling the mega binary took
about 5ms. After some tweaks I got it down to just above 4ms, but felt the loss
in readability was not worth the performance gain considering it should be
plenty fast for a web server as is.

The main culprit for performance issues is formating and concatenating strings.
Disassembling the gigabinary takes about 280ms (220ms on release build). If
output is piped to /dev/null, it drops to 90ms (40ms on release build). The
mos-6502 shouldn't even be able to handle a binary of that size, as the offset
won't fit to 16 bits.

I tried out rayon for parallilizing the string operations and it did yield
promising results, but only beyond a certain point. The giga binary benchmark
went down about 50%. Unfortunately the mega binary went up about 50%. A more
thought out parallelization strategy may be beneficial.

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

## Level 3

I got most of this done by early Saturday of week 1, meaning I still had over a
week left before the deadline. In terms of work hours it was probably around 3
full days. The themes of level 3 was correctness and expanding usability. The
most notable change was the addition of a frontend, where one could use the
system through a web browser.

The frontend was made with Askama templates on the backend and HTMX in the
browser to add some interactivity without having to do a full page load. I'm
particularly satisfied with the simplicity of it all. HTMX has been something
I've been circling for a while and this gave me an opportunity to try it out. In
about three lines of html, I got all the interactivity I wanted. You can upload
files and have the bytes decoded as hexadecimal digits and have your hexadecimal
text disassembled. The Askama templates I used to to generate the html had a few
positive surprises. Askama uses rust macros to check the template variables
compile time. It can alert you if a field is unused or you are missing one. The
templates also get baked into the binary, so packaging the application was
simple. I'll likely use both HTMX and Askama again. While the frontend isn't the
fanciest site on the internet, I find it functional and beautifully simple.

I initially used utoipa to generate openapi documentation. While writing the
frontend I realized it wasn't quite as ergonomic as I had hoped. I spent a lot
of time fighting it, to the point where I probably could've just manually
written the openapi documentation at that point. It was cumbersome, had subpar
error messages and was missing some compile time safety that I've come to expect
from rust. The system compiled just fine even if the openapi spec was
incomplete, giving a runtime warning if you open the swagger page. After trying
a few things, I switched axum and utoipa for poem, which comes with a first
party openapi generator. I liked how poem structured the routes and how little
additional work the api docs required.

Finally on the frontend/presentation side, I set the whole thing up in oracle
cloud through docker. The details are explained [here](#CI).

On the correctness side, I was most concerned about the big match statement
which decodes opcodes. Since it was mostly manually written, I was certain there
was a mistake in there that simply wasn't caught by the test binaries. To check,
I first scraped the html table from the reference site, parsed it into a csv
file and used that and python to generate a "megabinary" with one of each
opcode. I then fed this to the sample implementation and added a test similar to
the ones for both existing test binaries. Lo and behold, I had made a few errors
while transcribing the table. Fix those and felt good about myself. I also added
tests for the newly created frontend, which was surprisingly easy because it's
mostly template rendering.

I benchmarked the disassembly function and used cargo-flamegraph to see where
the time is going. You can read more about that [here](#Performance). I did a
couple of trivial optimizations here like removing unnecessary clonings where I
can use borrows, but left it mostly unchanged.

Some of the level 3 tests are a good idea and in hindsight I think the optimal
amount of effort for this assignment would be somewhere between levels 2 and 3.
The frontend was not something I would do by default given the initial
assignment, I just felt like trying it out and since I have plenty of time I saw
no harm in this detour.
