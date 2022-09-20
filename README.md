# Twilight Cache

A backend-agnostic Discord cache implementation for the Twilight ecosystem

It provides a `Cache` trait that provides methods to get data from the cache and a `Backend` trait used to add support
for a backend, refer to the documentation of each trait for more

## Support for libraries other than Twilight

This doesn't depend tightly on Twilight, you can easily fork this and change the Twilight models used in it