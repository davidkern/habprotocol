# HABprotocol

Shared protocol definitions for the HAB project.

Used by:

* [habctl](https://github.com/davidkern/habctl) - backend server
* [habux](https://github.com/davidkern/habux) - frontend web client
* [habbcm](https://github.com/davidkern/habbcm) - embedded body control module

## Versioning

The package version of this project is used at runtime to indicate protocol version
and controls client behavior when an existing client connects to a newer server version.

A version is assigned in semver format `major`.`minor`.`patch`, and corresponds to the
following client behavior:
 
1. A change in the `patch` level of the server WILL NOT cause a client to reload. It
   is expected to be able to continue to run using its older protocol structures.
2. A change in the `minor` level of the server indicates that new functionality has
   been added to the protocol and clients SHOULD reload when it is not inconvenient to
   the user (e.g. when idle, or when user accepts).
3. A change in the `major` level of the server indicates that the protocol is now
   incompatible and the client MUST reload.

During development of the project, it is expected that there will be many breaking
changes made to this protocol.  Thus the `major` level will be the most frequently
updated level and is not indicative of release-worthiness of the project.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
