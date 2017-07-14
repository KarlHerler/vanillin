# Vanillin discord bot

This is a very simple discord bot for grabbing links from db.vanillagaming.org.
The bot is written in the Rust programming language and licensed under the MIT license.


## Running

This project produces a binary systems application which needs to run as long as the
bot is to remain functional. The binary takes a `DISCORD_TOKEN` environmental variable with the
discord token used for this bot instance


## Building

This project uses the rust [cargo crates](https://crates.io/) system as a build
environment so building this project is as easy as: `cargo build`. A GNU Make config
is also included so running: `make` will also build the software

## Dependencies

 - [discord](https://crates.io/crates/discord) - A library for interacting with the Discord API (please note that this project has external dependencies which need to be satisfied)
 - [hyper](https://crates.io/crates/hyper)     - A http client (and server) library. Used to look up the correct thing from vanillagaming


## TODO and Contribution

This project is open to contributions of all forms, from in code changes to reporting issues and requesting features. Here is a small list of things to improve for future versions:

 - Add a struct for the vanillagaming result (to introspect the result type, the artifact returned e.t.c), this could be used to generate better response texts
 - Add an easier UI for adding it to a channel (this is dependent on the deployment but we can at least include some static HTML page scaffold)
 - Consider supporting [embedded rich objects](https://discordapp.com/developers/docs/resources/channel#embed-object) for more fancy messages
