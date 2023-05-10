# Implementation of Choclo kernels in Rust

I started this repo to experiment with Python modules written in Rust, using
[PyO3][pyo3], [Maturin][maturin], the [`ndarray`][ndarray] and
[`numpy`][numpy] crates, and [`rayon`][rayon] for parallelizing the runs.

I choose to implement the forward model of the vertical component of the
gravity acceleration of point sources on a set of computation points, inspired
in the [Choclo][choclo] function [`choclo.point.gravity_u`][gravity_u].

> **Warning**
>
> This code is just an experiment, it's note being tested or actively
> maintained. Feel free to use it to learn and as a starting point to build your
> own Python modules in Rust.

## How to build and run

Clone this repo:

```
git clone https://github.com/santisoler/choclo-rs
cd choclo-rs
```

Install [`maturin`][maturin] and build the package with:

```
maturin develop --release
```

> We use the `--release` option to enable Rust optimizations. If you are
> playing around with your code, you can speed up the build by omitting the
> `--release` option, but expect a performance hit.
>
> **Make use of the `--release` option if you are benchmarking the code**.

Now you'll be able to import `choclors` from Python:

```python
import choclors
```

For example:

```python
import numpy as np
import choclors

easting = np.array([0.])
northing = np.array([0.])
upward = np.array([0.])
points = np.array([0., 0., -10.])
masses = np.array([1e6])
result_rust = choclors.points_gz(easting, northing, upward, points, masses)
```


## License

This is free software: you can redistribute it and/or modify it under the terms
of the BSD 3-clause License. A copy of this license is provided in
[LICENSE](LICENSE).

[pyo3]: https://pyo3.rs
[maturin]: https://www.maturin.rs
[ndarray]: https://docs.rs/ndarray
[numpy]: https://docs.rs/numpy
[rayon]: https://docs.rs/rayon
[choclo]: https://www.fatiando.org/choclo
[gravity_u]: https://www.fatiando.org/choclo/latest/api/generated/choclo.point.gravity_u.html#choclo.point.gravity_u
