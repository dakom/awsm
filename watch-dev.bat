@echo off
cd examples/crate
cargo check
watchexec -w ../../lib/src -w src cargo check
