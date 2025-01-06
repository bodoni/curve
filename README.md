# Curve [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides curves.

## Example

```rust
let x = curve::bezier::Linear::new(1.0, 5.0);
let y = curve::bezier::Linear::new(2.0, 3.0);
let points = x.trace(3).zip(y.trace(3)).collect::<Vec<_>>();
assert_eq!(points, vec![(1.0, 2.0), (3.0, 2.5), (5.0, 3.0)]);
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/bodoni/curve/workflows/build/badge.svg
[build-url]: https://github.com/bodoni/curve/actions/workflows/build.yml
[documentation-img]: https://docs.rs/curve/badge.svg
[documentation-url]: https://docs.rs/curve
[package-img]: https://img.shields.io/crates/v/curve.svg
[package-url]: https://crates.io/crates/curve
