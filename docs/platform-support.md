# Sweden Platform Support

The project designs portable library boundaries for:

- Linux;
- Windows;
- FreeBSD and NetBSD, with additional BSD targets as Rust distributes them;
- macOS;
- Android;
- iOS.

Portable crates avoid implicit filesystem, environment, socket, clock, thread,
and process assumptions. Platform-specific behavior belongs in focused adapter
crates.

`no_std` compilation proves only that domain and request logic can operate
without the standard library. It does not claim that DNS, TCP, TLS, trust
stores, clocks, or credentials work without a platform implementation.

Aesynx support is future work. The current `no_std` core and caller-supplied
transport boundary reserve that path without referring to unfinished Aesynx
interfaces today.
