# Implementation of Choclo kernels in Rust

I started this repo to experiment with Python modules written in Rust, using
[PyO3][pyo3], [Maturin][maturin] and the [`ndarray`][ndarray] and
[`numpy`][numpy] crates.

I choose to implement the forward model of the vertical component of the
gravity acceleration of point sources on a set of computation points, inspired
in the [Choclo][choclo] function [`choclo.point.gravity_u`][gravity_u].

> **Warning**
>
> This code is just an experimentation, it's note being tested or actively
> maintained. Feel free to use it to learn and as a starting point to build your
> own Python modules in Rust.

## License

This is free software: you can redistribute it and/or modify it under the terms
of the BSD 3-clause License. A copy of this license is provided in
[LICENSE](LICENSE).

[pyo3]: https://pyo3.rs
[maturin]: https://www.maturin.rs
[ndarray]: https://docs.rs/ndarray
[numpy]: https://docs.rs/numpy
[choclo]: https://www.fatiando.org/choclo
[gravity_u]: https://www.fatiando.org/choclo/latest/api/generated/choclo.point.gravity_u.html#choclo.point.gravity_u
