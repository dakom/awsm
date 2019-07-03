[![Build Status](https://travis-ci.org/dakom/awsm.svg?branch=master)](https://travis-ci.org/dakom/awsm)

## [DEMO](https://awsm.netlify.com/)

## Status

Just the beginning, and learning Rust...but it works! 

## About

awsm is mid-level crate for wasm, especially gamedev / immersive experience things

Overall, the approach with this library is similar in spirit to [gloo](https://github.com/rustwasm/gloo) - that is to say, it aims to bridge the gap between the auto-generated WebIDL-powered bindings web-sys provides, and the type of code we'd typically consider a real starting point in web apps.

It's also just my personal stomping grounds to figure stuff out, and is therefore more free to err on the side of "is this good enough for now" vs. "is this the right api, is it modular enough, etc."

There's a few major areas under development, but lots of stuff is done :) Check it out!

## Developing

There are two steps:

1. run `watch-dev.bat` in the main folder.

Yeah, I'm on windows - sorry! This will watch for changes to rust sources and rebuild / bindgen to create the wasm on change

Usually this is the window to look at for iterative feedback

This is _required_ to cause live-reloading in webpack (`npm start` isn't enough)

2. cd examples && npm start

This will start up the webpack dev server (and it will restart when rust recompiles _successfully_)

The reason these are kept separate is that I found the best way is to keep them in completely separate windows. The webpack dev server generates its own noise and does weird coloring stuff.

Also, there could theoretically be different settings for "watch-dev" i.e. to only run `cargo check` etc. but I haven't really needed that yet

Anyway, made sense to keep the steps separate :)

3. Edit examples/Cargo.toml to switch between webgl1 and webgl2

It's changing one character, but you can only test one at a time (since it's a compile-time flag)

Other apps aren't limited by this - it's just for the examples (not library)

## Release

For the app example - `npm run bundle` in the examples folder should cover it

But make sure `wasm-opt` is installed by downloading the latest release from [binaryen](https://github.com/WebAssembly/binaryen/releases) and putting it in the path somewhere

For the library - it's just normal cargo package/publish in the `lib/` folder

## Live Coding 

As a way to self-motivate getting over the learning curve (this is my first Rust project), I thought it might help to livestream the coding+learning sessions, and also archive them in a playlist for a look-back.

On twitch: https://www.twitch.tv/dakomz
