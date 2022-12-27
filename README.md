# WG Display

[![Cargo test](https://github.com/eliabieri/wg_display/actions/workflows/cargo_test.yml/badge.svg)](https://github.com/eliabieri/wg_display/actions/workflows/cargo_test.yml)

## All that goes on in your city at one glance

## Extensible, open-source and connected to the local community

![WG Display image front](docs/images/wg_display.jpg)

## Building the documentation (rustdocs)

```bash
make docs
```

This generates three seperate documentations, one for each crate

[app](app/target/doc/wg_display/index.html): ```app/target/doc/app/index.html```

[common](common/target/doc/common/index.html): ```common/target/doc/common/index.html```

[frontend](frontend/target/doc/frontend/index.html): ```frontend/target/doc/frontend/index.html```

## Safety

This project uses `#[forbid(unsafe_code)]` in all crates to ensure that no `unsafe` Rust is ever added to the project.
