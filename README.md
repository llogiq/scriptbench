# scriptbench

We are searching for a scripting language. It should be reasonably fast, safe and allow us to
easily embed functions into our engine. It should also be able to handle types without too complex
marshalling.

### languages

* **lua** has two implementations: hlua and rlua. Both are no longer worked on and embed the C
library with all of its downsides regarding safety. Untrusted scripts could well crash the program,
introduce unsound operations or at least slow down operation dramatically. On the other hand, it's
also known to be one of the fastest scripting languages out there. hlua wraps Lua 5.2 and rlua
wraps lua 5.4 (and takes safety a bit more seriously). The latter has 64 bit integers and 64 bit
floats (instead of just floats), which maps well to our domain.

The lua runtime is a piece of art. Even if we end up creating a new language, we should take some
cues from its implementation.

* **gluon** is a statically typed functional language. Unfortunately I couldn't get it to compile,
which has prevented me from benchmarking it.

* **dyon** is a dynamically typed scripting language. The syntax looks weird, the error messages 
are bare and the docs less than stellar, but I was able to implement a small function embedding 
example to benchmark.

* **rhai** is yet another dynamically typed language that uses an AST based interpreter and offers
no_std compatibility. It is likely slower than the other languages, and its embedding API does
not lend itself to fast execution of a call to a rhai-defined function.

* **koto** is a dynamically typed language meant for live coding. It's got a byte code interpreter
and should be fairly easy to embed.
