# nostd_cursor

Since it's impossible to use `std::io::Cursor<T>` in a no_std environment I decided to create a *very* *very* simple implementation that supports [byteorder](https://crates.io/crates/byteorder) out of the box since that's how I always use `Cursor`.

## Usage

### Cursor WITHOUT byteorder
A normal cursor can be found in `nostd_cursor::cursor::Cursor`

### Cursor WITH byteorder
Either use `nostd_cursor::LECursor<T>` or `nostd_cursor::BECursor<T>`.

## Contribute

At this point I've only trivially implemented `Cursor::read_exact()`. If you need anything else that is implemented in the std-version feel free to create a pull request! I'm only planning to add more features when I need them for my personal projects.