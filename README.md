# xkcd-rs

<table>
    <tr>
        <td><strong>Linux / OS X</strong></td>
        <td><a href="https://travis-ci.org/indiv0/xkcd-rs" title="Travis Build Status"><img src="https://travis-ci.org/indiv0/xkcd-rs.svg?branch=master" alt="travis-badge"></img></a></td>
    </tr>
    <tr>
        <td colspan="2">
            <a href="https://indiv0.github.io/xkcd-rs/xkcd" title="API Docs"><img src="https://img.shields.io/badge/API-docs-blue.svg" alt="api-docs-badge"></img></a>
            <a href="https://crates.io/crates/xkcd" title="Crates.io"><img src="https://img.shields.io/crates/v/xkcd.svg" alt="crates-io"></img></a>
            <a href="#License" title="License: MIT/Apache-2.0"><img src="https://img.shields.io/crates/l/xkcd.svg" alt="license-badge"></img></a>
            <a href="https://coveralls.io/github/indiv0/xkcd-rs?branch=master" title="Coverage Status"><img src="https://coveralls.io/repos/github/indiv0/xkcd-rs/badge.svg?branch=master" alt="coveralls-badge"></img></a>
        </td>
    </tr>
</table>

Rust bindings for the XKCD web API.

# Table of Contents

* [Usage](#usage)
* [Contributing](#contributing)
* [Credits](#credits)
* [License](#license)

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
xkcd = "0.2"
```

And in your `lib.rs` or `main.rs`:

```rust
extern crate xkcd;
```

See the [API docs][api-docs] for information on using the crate in your library.

## Contributing

Contributions are always welcome!
If you have an idea for something to add (code, documentation, tests, examples,
etc.) feel free to give it a shot.

Please read [CONTRIBUTING.md][contributing] before you start contributing.

## Credits

The higher-level API design for this library is inspired by the
[slack-rs/slack-rs-api(https://github.com/slack-rs/slack-rs-api) library.

The list of contributors to this project can be found at
[CONTRIBUTORS.md][contributors].

## License

xkcd-rs is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE][license-apache], and [LICENSE-MIT][license-mit] for details.

[api-docs]: https://indiv0.github.io/xkcd-rs/xkcd
[contributing]: https://github.com/indiv0/xkcd-rs/blob/master/CONTRIBUTING.md "Contribution Guide"
[contributors]: https://github.com/indiv0/xkcd-rs/blob/master/CONTRIBUTORS.md "List of Contributors"
[license-apache]: https://github.com/indiv0/xkcd-rs/blob/master/LICENSE-APACHE "Apache-2.0 License"
[license-mit]: https://github.com/indiv0/xkcd-rs/blob/master/LICENSE-MIT "MIT License"
