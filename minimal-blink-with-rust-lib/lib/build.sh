#!/bin/sh

rustc --target thumbv6m-none-eabi --crate-type staticlib app.rs

