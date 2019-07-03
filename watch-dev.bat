@echo off
cd examples
watchexec -w ../src -w crate/src npm run _rust:build:dev
rem watchexec -w ../src -w crate/src npm run _rust:build:dev:nowarnings
