@echo off
cd examples
watchexec -w ../lib/src -w crate/src npm run _rust:build:dev
rem watchexec -w ../lib/src -w crate/src npm run _rust:build:dev:nowarnings
