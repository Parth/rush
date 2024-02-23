# rush - the RUst SHell 

🚧 barely functional wip 🚧

A rich shell with great defaults, similar to fish. Aiming to provide:
+ syntax highlighting
+ strong historical suggestion & tab completion experience
+ vi mode
+ terminal multiplexer

All in a no dependency, pure-rust, tiny binary (one day).

Rush also takes a unique stance towards power user configuration:
+ no startup files are parsed upon startup
+ power users can configure their shell by pulling rush as a library dependency 
and configuring their shell in code
+ plugins for prompts and auto completions can be defined as simple rust functions 
and distributed through cargo (rather than in a scripting language and distributed 
ad-hocly)

These values aim to provide an improved experience for both types of users:
+ faster -- default systems programming language rather than a scripting one.
+ reliable -- loosely coupled tools, configuration files and scripting languages don't have strong guarantees around correctness. Rust and Cargo have strong enforcement of contracts and conventions around evolving code.
+ portable -- the way you get binaries, plugins and perform configurations varies significantly based on the platform. The rust programming language and cargo's behavior varies significantly less.
+ rich experience -- if you're writing a plugin in rust, it's trivial to add a library from Cargo's massive collection of community libraries.
