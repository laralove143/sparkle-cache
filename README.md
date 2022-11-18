# Sparkle Cache

[GitHub](https://github.com/laralove143/sparkle-cache)
[crates.io](https://crates.io/crates/sparkle-cache)
[docs.rs](https://docs.rs/sparkle-cache/latest)

A backend-agnostic Discord cache implementation for the Twilight ecosystem

It provides a `Cache` trait that provides methods to get data from the cache and a `Backend` trait used to add support
for a backend, refer to the documentation of each trait for more

## Usage

This crate is for adding support for a backend, if you just need to use the cache, you should use one of the crates:

- [Sparkle Cache Postgres](https://github.com/laralove143/sparkle-cache-postgres)
- Please create a PR to add your crate to this list

## Compatibility

The models don't use any arrays and every field is a primitive type, this makes it compatible with schematic backends
out of the box

## Incompleteness

Only the data from events are cached, though it's on the to-do list to add support for data that requires API methods

This means this data can't be cached for now:

- Private channels
- Bans
- Auto moderation rules
- Integrations
- Scheduled events
- Invites
- Webhooks
- Missing data that you can create a PR to add to this list

## Support for libraries other than Twilight

This doesn't depend tightly on Twilight, you can easily fork this and change the Twilight models used in it

## Features

### Tests

Enables the testing module, it's intended for libraries implementing traits in this library, and it should be enabled
only under `[dev-dependencies]`, for example

```toml
[package]
name = "sparkle-cache-some-backend"
[dev-dependencies]
sparkle-cache = { version = "x", features = ["tests"] }
[dependencies]
sparkle-cache = "x"
```

If the test error is related to this crate, please create an issue

The tests currently don't cover stickers because
of [a bug in Twilight](https://github.com/twilight-rs/twilight/issues/1954)

## Your help is needed

Any feedback or bug reports will be very useful in further development, the code is in a working state with all
essential methods done but there's many possible additions that will be added as you request them!
