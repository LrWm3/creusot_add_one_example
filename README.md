# creusot example set-up

Example usage of [creusot](https://github.com/creusot-rs/creusot) as per [Quickstart Guide](https://creusot-rs.github.io/creusot/guide/).

## Getting started

- set-up [creusot](https://github.com/creusot-rs/creusot).
- install the [creusot ide vscode extension](https://marketplace.visualstudio.com/items?itemName=creusot-rs.creusot-ide).
- following the guide in the vscode extension, install the [creusot-ide](https://github.com/creusot-rs/creusot-ide).

## Purpose

Formal verification methods are a nice in-between linting and unit testing. despite this, formal verification tools are rarely used.

In the future, I believe these tools will rise to a similar level of popularity as unit testing and linting. We see people trying to generate unit tests automatically, but at that point I would think it would make more sense to simply leverage formal verification tools.

Rust as a language is relatively concise and strict when it comes to safe rust, making it potentially a more amendable target for formal verification methods.

There are two available tools for verification in rust, prusti and creusot.

prusti is by far the more ergonomic of the two, however, it sees significantly less development activity than creusot. Additionally, prusti is missing verification for the standard rust library, meaning these verification methods must be written by users in order to get started.

creusot sees more development, but lacks many of the QOL features of prusti.

- creusot requires being on the nightly build of rust
- creusot requires being built with the same toolchain as the target project
- creusot requires installing ocaml and why3
- vscode integration is somewhat confusing, with multiple options for frontends, whycode & creusot-ide.
- creusot-ide requires changes to `Cargo.toml` and `.vscode/settings.json` in order to work.
- creusot-ide requires the creusot-lsp be installed separately
- creusot-ide depends on creusot-lsp which I couldn't get to build
- whycode not "easy" to find.

Despite this friction, using a development container, we might be able to get developers set up relatively painlessly to use creusot.

additionally, it might be possible to set up a macro or something which would allow building on `stable` rust for production releases with the creusot stuff omitted.

I would also like to see if [cursor](https://www.cursor.com/) makes it easier to write proofs.

## Running

From the terminal, run:

```
cargo creusot prove -i
```

And this will prove.

## creusot-ide wouldn't build :(

While I was able to get why3 to work, the VSC language server would not build.

I could run the prover but coudln't use the linter.

I raised a ticket around this issue ()

### Whycode instead?

Perhaps this is a better front-end. Set-up steps are much simplier.

Unfortunately I couldn't figure out how to set this up either. the readme states "Setup the server and client like shown above" but the available link does not seem to work.

So in the end I could not get why3 or creusot working with anything besides the default GTK frontend. I will likely need to reach out to the community at a later point to understand what I should be doing.

> Later, I returned to this; the error indicated a missing config file. I added the config file to `~/.why3.conf`. The server would then run. However, I could not figure out how to get it to run my proofs. Perhaps I needed to run creusot to generate my why3 files first and then run it. Not sure.

## Conclusion

While prusti is comparatively simple to get started with, it's missing contracts for the standard library, meaning it cannot really be used.

Creusot is promising, but I was unable to get vscode integration working for it, which I consider a hard requirement for any verification tools.

In the end verification tools are still new and don't integrate well into vscode, at least for a novice like me who is new to the space.

Honestly I think it would be easier to personally finish writing the contracts for the standard rust library for prusti than use creusot.
