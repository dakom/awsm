[![Build Status](https://travis-ci.org/dakom/awsm.svg?branch=master)](https://travis-ci.org/dakom/awsm)

## [DEMO](https://dakom.github.io/awsm)

## About

awsm is a collection of helpers for doing wasm in Rust

There's two approaches that exist here:

1. incubating and testing out ideas that might move to [gloo](https://github.com/rustwasm/gloo) at some point

An example of this would be the `start_raf_ticker()` function, as well as most of the direct `webgl` functions.

Moving these into `gloo` and deprecating them here would be considered a good thing! If/when that happens, the original function might be kept around as a backwards compatible shim. 

2. opinionated helpers that are a bit _too_ opinionated to be considered universal defaults

An example of this would be the `webgl_renderer`. 

It makes sane choices about how to cache attribute lookups, for example, but not every project would want that hardcoded.

## Status

Just the beginning, and learning Rust... come back in a year or so?

## Developing

There are two steps:

1. run `watch-dev.bat` in the main folder.

This will watch for changes to rust sources and rebuild / bindgen to create the wasm on change

Usually this is the window to look at for iterative feedback 

2. cd examples && npm start

This will start up the webpack dev server (and it will restart when rust recompiles _successfully_)

The reason these are kept separate is that I found the best way is to keep them in completely separate windows. The webpack dev server generates its own noise and does weird coloring stuff.

There could theoretically be different settings for "watch-dev" i.e. to only run `cargo check` etc. but I haven't really needed that yet

Anyway, made sense to keep the steps separate :)

Sorry `watch-dev` is a bat. Could easily be made into `.sh` or whatever. Also it's not in npm scripts since it's nice to just double-click it ;) 

## Release

For the app example - `npm run bundle` in the examples folder should cover it

But make sure `wasm-opt` is installed by downloading the latest release from [binaryen](https://github.com/WebAssembly/binaryen/releases) and putting it in the path somewhere

For the library - it's just normal cargo package/publish in the `lib/` folder

## Live Coding 

As a way to self-motivate getting over the learning curve (this is my first Rust project), I thought it might help to livestream the coding+learning sessions, and also archive them in a playlist for a look-back.

On youtube: https://www.youtube.com/channel/UCGKhwtFOUlzj4VILDU0efkQ/live

On twitch: https://www.twitch.tv/dakomz
