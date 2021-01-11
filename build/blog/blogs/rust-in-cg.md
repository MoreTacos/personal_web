# Reasons Rust is Ideal for CG Research
*Oct 22, 2020*

*This article is slowly being updated as I learn more about Rust and CG*

While these points apply for all kind of development in general, a couple of them are particularly prevalent in CG. I'll get to explaining that in a bit. But first a couple of words on the current state of graphics programming in rust.
Rust has been rated the most loved language for 4 years in a row by Stack Overflow's [developer survey](https://insights.stackoverflow.com/survey/2020#technology-most-loved-dreaded-and-wanted-languages-loved). The main reason being that it solves a couple pain points that are normally difficult to deal with in c++. While Rust isn't perfect, everything that is annoying in Rust is but a drop in a bucket compared to the million little pains about c++ development that just add up, such as the following:

- memory safety problems
- a complicated choice for build systems
- no package manager
- portability issues
- confusing error messages
- header files are confusing
- memory safety problems
- arcane implementation

While these points apply for all kind of development in general, a couple of them are particularly prevalent in CG. I'll get to explaining that in a bit. But first a couple of words on the current state of graphics programming in rust.

All the fundamental API's are written in C and C++. So people write rust wrappers(glium, luminance, ash, vulkano, glx-hal+rendy) that make life nice and safer to varying degrees. That said it is still very low level, and pursuing complete safety can be expensive and difficult. For more information check out [this](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019) article on a guide to Rust graphics programming. Now onto the points that might push Rust into mainstream CG development.

## Memory Safety Problems
The advantage of safety in Rust isn't just about security vulnerabilities (which hardly applies for graphics programming). Instead it can enable faster development and increase productivity. In C++, page faults and data races take an eternity to fix, and as a result complicated concurrency models are far less appealing to implement. In contrast these kinds of problems in Rust are typically local and easier to fix. A quote from the Rust book's foreword states

*"Traditionally, this realm of programming is seen as arcane, accessible only to a select few who have devoted the necessary years learning to avoid its infamous pitfalls. Rust breaks down these barriers by eliminating the old pitfalls and providing a friendly, polished set of tools to help you along the way. The language is designed to guide you naturally towards reliable code that is efficient in terms of speed and memory usage"*

With a more simple model for concurrency, way more people (like me) can have a go at writing high-performance CG algorithms without having to fear hours of debugging. How much potential is currently locked in many CG enthusiasts due to the barrier of entry required for understanding the weird quirks of C and C++? The concepts of graphics programming on their own can already be overwhelming for beginners transitioning from higher level languages like Python or JS.

## Cargo: Rust's build system & package manager
Oh man! Talk about a two for one. C++'s variety of build systems (and build systems that build build systems: talking to you CMake) is so confusing it is in part responsible on why I transitioned to Rust. And c++'s header files weren't easy either. What a nightmare!

Rust installations comes with cargo. Cargo offers a flow that is orders of magnitudes easier for beginners like me. And because dependencies, tests, and documentation are available by default, they are widely used.

## Helpful Compiler Messages
Another great feature is how helpful the compiler can be. Compiler messages for c++ code, for example, are notoriously difficult to decipher. While clang has made very good improvements, rust's compiler is just so tremendously helpful. (a couple of examples)

## Bench-marking tools
I've heard good things, but never used it myself. Take a look [here](https://gist.github.com/KodrAus/97c92c07a90b1fdd6853654357fd557a) and [here](https://blog.anp.lol/rust/2016/07/24/profiling-rust-perf-flamegraph/) for more info.

## The Downsides
Very briefly, there are a couple of disadvantages that come with Rust:

- slow compile times
- float ordering problems
