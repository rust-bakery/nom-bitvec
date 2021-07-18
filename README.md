# nom-bitvec

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Join the chat at https://gitter.im/Geal/nom](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/Geal/nom?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Crates.io Version](https://img.shields.io/crates/v/nom-bitvec.svg)](https://crates.io/crates/nom-bitvec)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.51.0+-lightgray.svg)](#rust-version-requirements)

This crate provides input types for [nom parser combinators](https://crates.io/crates/nom)
using [bitvec](https://crates.io/crates/bitvec).
With those, you can use common nom combinators directly on streams of bits.
