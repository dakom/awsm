[![Build Status](https://travis-ci.org/dakom/awsm.svg?branch=master)](https://travis-ci.org/dakom/awsm)

## [DEMO](https://dakom.github.io/awsm)

## About

awsm is a collection of helpers for doing wasm in Rust

There's two approaches that exist here:

1. incubating and testing out ideas that might move to [gloo](https://github.com/rustwasm/gloo) at some point

An example of this would be the `start_raf_ticker()` function, as well as most of the direct `webgl` functions.

Moving these into `gloo` and deprecating them here would be considered a good thing!

2. opinionated helpers that are a bit _too_ opinionated to be considered universal defaults

An example of this would be the `webgl_renderer`. 

It makes sane choices about how to cache attribute lookups, for example, but not every project would want that hardcoded.

## Status

Just the beginning, and learning Rust... come back in a year or so?

## Live Coding 

As a way to self-motivate getting over the learning curve (this is my first Rust project), I thought it might help to livestream the coding+learning sessions, and also archive them in a playlist for a look-back.

On youtube: https://www.youtube.com/channel/UCGKhwtFOUlzj4VILDU0efkQ/live

On twitch: https://www.twitch.tv/dakomz
