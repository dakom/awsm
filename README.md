[![Build Status](https://travis-ci.org/dakom/awsm.svg?branch=master)](https://travis-ci.org/dakom/awsm)

## [Live Feature Tests](https://awsm.netlify.com/)

## About

awsm is a work-in-progress 3d game engine for the web, built in Rust and WebGL2. It's fast (or will be, hopefully).

It is planned to be similar in concept to [amethyst](https://amethyst.rs/) and other game engines that use an Entity Component System and glTF - with the major difference being that awsm is focused _solely_ on the web.

## Architecture

awsm is split into two crates:

* [awsm_web](crates/web) (very far along) - generic helpers for wasm on the web. Intended to be used independently, even for lightweight feature-specific needs.
[CRATES.IO](https://crates.io/crates/awsm_web) - [DOCS](https://docs.rs/awsm_web)


* [awsm](crates/engine) (r&d / pre-production) - the engine itself
[CRATES.IO](https://crates.io/crates/awsm) - [DOCS](https://docs.rs/awsm)


## Status

Nothing yet! Only split awsm_web out which is what used to be here ;) Check back in a few months for the engine itself...

## Live Coding 

As a way to self-motivate getting over the learning curve, I thought it might help to livestream the coding+learning sessions, and also archive them in a playlist for a look-back.

On twitch: https://www.twitch.tv/dakomz

## Additional Demos

* [Bunnymark](https://dakom.github.io/rust-bunnymark/)