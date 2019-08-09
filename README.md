[![Build Status](https://travis-ci.org/dakom/awsm.svg?branch=master)](https://travis-ci.org/dakom/awsm)

## About

awsm is a 3d game engine for the web, built in Rust and WebGL2. It's fast.

Conceptually, it's similar to [Amethyst](https://amethyst.rs/) and other game engines that use an Entity Component System and glTF - with the major difference being that awsm is focused _solely_ on the web. This means it can conceptually be lighter, faster, and more ergonomic in this niche. That's the idea anyway... 

## Status

Nothing yet! Only split awsm_web out which is what used to be here ;) Check back in a few months for the engine itself...

## Demos

* ### [Feature Tests](https://awsm.netlify.com/) 
* ### [Bunnymark](https://dakom.github.io/rust-bunnymark/)

## Architecture

awsm is split into two crates:

* [awsm](crates/engine) - the engine itself
[CRATES.IO](https://crates.io/crates/awsm) - [DOCS](https://docs.rs/awsm)

* [awsm_web](crates/web) - generic helpers for wasm on the web. Intended to be used independently, even for lightweight feature-specific needs.
[CRATES.IO](https://crates.io/crates/awsm_web) - [DOCS](https://docs.rs/awsm_web)


## Live Coding 

As a way to self-motivate getting over the learning curve, I thought it might help to livestream the coding+learning sessions, and also archive them in a playlist for a look-back.

On twitch: https://www.twitch.tv/dakomz