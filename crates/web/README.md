## About

awsm_web is mid-level crate for rust-powered wasm, especially gamedev / immersive experience things that need higher performance and/or no GC

Although it's an independant crate, it's primarily used as a building block in the [awsm engine](https://github.com/dakom/awsm).

## Description 

The approach with this library is similar in spirit to [gloo](https://github.com/rustwasm/gloo) - that is to say, it bridges the gap between the auto-generated WebIDL-powered bindings web-sys provides, and the type of code we'd typically consider a real starting point in web apps.

The goal is to keep it very low level and low-cost abstraction that is _almost_ universal. However, _almost_ universal is not without opinions. For example, the webgl wrapper does a ton of up-front caching and stores local state to avoid making gl calls unnecessarily - something most projects would do, but not all (e.g. if all locations are hardcoded). 

## Features

The reason for splitting this crate out is so that it can be used independantly (and so it doesn't carry the semver burden of rapidly changing downstream crates). 

Features are therefore used extensively to keep dependencies minimal. The default is to have them all turned on, so make sure to disable them and only use what you need. For example:

```
[dependencies.awsm_web]
features = ["loaders"]
default-features = false 
```