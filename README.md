# Rote

![status: experimental](https://img.shields.io/badge/status-experimental-orange)
[![crate](https://img.shields.io/crates/v/rote.svg)](https://crates.io/crates/rote)
[![docs](https://docs.rs/rote/badge.svg)](https://docs.rs/rote)

Pure Rust implementation of exact arithmetic and algorithms for computer geometry applications.
(cf. [CGAL](https://www.cgal.org/)).

This crate requires Rust 1.51 or higher.

## WARNING

This implementation prioritizes safety and correctness over performance, but can not yet claim
correctness nor suitibility for any task other than experimentation. Implementors requiring this
functionality are encouraged to use bindings to a proven C-library until this implementation has
been shown to be robust.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## References

### Papers

 * State-Effect Pattern: https://www.cs.cornell.edu/~wmwhite/papers/2008-Sandbox-Declarative.pdf
 * Fast, Exact, Linear Booleans: http://www.gilbertbernstein.org/resources/booleans2009.pdf
 * Manifold Dual Contouring: https://people.engr.tamu.edu/schaefer/research/dualsimp_tvcg.pdf
 * http://image.diku.dk/projects/media/morten.mikkelsen.08.pdf

### Crates

 [hmeyer](https://crates.io/users/hmeyer) crates

 * https://crates.io/crates/tessellation
 * https://crates.io/crates/bbox
 * https://crates.io/crates/implicit3d
 * https://crates.io/crates/stl_io
 