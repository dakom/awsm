[![Build Status](https://travis-ci.org/dakom/awsm.svg?branch=master)](https://travis-ci.org/dakom/awsm)

## [DEMO](https://awsm.netlify.com/) - [CRATES.IO](https://crates.io/crates/awsm) - [DOCS](https://docs.rs/awsm)

## Status

Still keeping it in minor version number for now, since the API might change - but lots of stuff is done :) Check it out!

## About

awsm is mid-level crate for wasm, especially gamedev / immersive experience things that need higher performance and/or no GC

Overall, the approach with this library is similar in spirit to [gloo](https://github.com/rustwasm/gloo) - that is to say, it aims to bridge the gap between the auto-generated WebIDL-powered bindings web-sys provides, and the type of code we'd typically consider a real starting point in web apps.

It's also just my personal stomping grounds to figure stuff out, both on the Rust side (it's my first project) and the raw webgl renderer side (second or third) and is therefore more free to err on the side of "is this good enough for now" vs. "is this the right api, is it modular enough, etc."

The various features pretty much match what is testable on the [demo](https://awsm.netlify.com/) and each one has a link to that example's source.


## Building 

Commands are run via npm in the `examples/` folder

**Development mode**

`npm start`

This will spin up the demo and open a browser with live reloading. Requires that [watchexec](https://github.com/watchexec/watchexec) be installed)

**Bundling** 

`npm run bundle` 

Really just a preliminary step for deployment, but useful for seeing how the optimization settings kick in too. Requires that [wasm-opt](https://github.com/WebAssembly/binaryen) be installed. _tip: just extract the zip and put it in the path somewhere._

**Deployment** 

`npm run deploy` 

Should fail for non-authorized users... Requires that [netlify-cli](https://www.netlify.com/docs/cli/) be installed)

## Live Coding 

As a way to self-motivate getting over the learning curve (as mentioned above - this is my first Rust project), I thought it might help to livestream the coding+learning sessions, and also archive them in a playlist for a look-back.

On twitch: https://www.twitch.tv/dakomz
