# nostd_cursor

Since it's impossible to use `std::io::Cursor<T>` in a no_std environment I decided to create a *very* *very* simple implementation that supports [byteorder](https://crates.io/crates/byteorder) out of the box since that's how I always use `Cursor`.

## Usage

This package comes by default with the byteorder-feature enabled.

### Cursor WITHOUT byteorder
A normal cursor can be found in `nostd_cursor::Cursor`, remember to set `default-features = false` in your Cargo.toml.

### Cursor WITH byteorder
Either use `nostd_cursor::LECursor<T>` or `nostd_cursor::BECursor<T>`.

## Contribute

Most of the API surrounding `std::io::Read` and `std::io::Write` isn't implemented yet. If there's something missing that you need feel free to submit a PR.