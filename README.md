# Twilight Cache

A backend-agnostic Discord cache implementation for the Twilight ecosystem

It provides a `Cache` trait that provides methods to get data from the cache and a `Backend` trait used to add support
for a backend, refer to the documentation of each trait for more

## Compatibility

The models don't use any arrays and every field is a primitive type, this makes it compatible with schematic backends
out of the box

## Incompleteness

The cache itself doesn't have access to the Discord API methods, it's on the to-do list to add support for data that requires API methods.

This means these data can't be cached for now:

- Bans
- Auto moderation rules
- Integrations
- Scheduled events
- Invites
- Webhooks
- Missing data that you can create a PR to add to this list

## Support for libraries other than Twilight

This doesn't depend tightly on Twilight, you can easily fork this and change the Twilight models used in it