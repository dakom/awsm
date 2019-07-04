@echo off
cd examples
rem watchexec -w ../src -w crate/src npm run _rust:build:dev
watchexec -w ../src -w crate/src npm run _rust:build:dev:nowarnings
