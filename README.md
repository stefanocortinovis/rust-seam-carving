# Rust Seam Carving

![CI](https://github.com/scortino/rust-seam-carving/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/scortino/rust-seam-carving/branch/master/graph/badge.svg?token=AAMV751422)](https://codecov.io/gh/scortino/rust-seam-carving)

A Rust executable to perform content-aware image resizing using [seam carving](https://en.wikipedia.org/wiki/Seam_carving).

## Installation

The executable can be installed using [Cargo](https://github.com/rust-lang/cargo/) as follows:

```console
git clone https://github.com/scortino/rust-seam-carving.git
cd rust-seam-carving
cargo install --path .
```

## Usage

The executable can be used from the command line as follows:

```console
rsc /path/to/image new_width new_height
```

## Running tests

Unit tests and integration tests can be run from the project directory as follows:

```console
cargo test --release
```

Longer integration tests generating examples of carved images starting from the ones initially present in the `img/` directory can be run as follows:

```console
cargo test --release -- --ignored
```

## Credits

This project was proposed by Professors C. Feinauer and F. Pittorino as part of their course [20602 - Algorithms](https://didattica.unibocconi.eu/ts/tsn_anteprima.php?cod_ins=20602&anno=2021&IdPag=6351).

The implementation closely follows the original algorithm presented by S. Avidan and A. Shamir in their paper [Seam carving for content-aware image resizing](https://dl.acm.org/doi/abs/10.1145/1275808.1276390) (2007).

Some of the unit tests used in the project are inspired by [this](https://www.cs.princeton.edu/courses/archive/fall13/cos226/assignments/seamCarving.html) Princeton University's assignment.
