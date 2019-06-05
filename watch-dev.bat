@echo off
cd examples
rem watchexec -w ../lib/src -w crate/src npm run _rust:build:dev
watchexec -w ../lib/src -w crate/src npm run _rust:build:dev:nowarnings
