# RouterOS API in Rust

routeros-proto is an implementation of the [RouterOS API](https://help.mikrotik.com/docs/display/ROS/API) protocol, 
without the transport layer. It's an attempt not only to implement encoding/decoding, but to add a more typed API as well.

While the crate is `no-std`, due to the protocol specifics (variable-length words & unknown sentence size),
it requires the `alloc` feature.

**Features:**

- [x] Stick to `no-std`
- [x] Command sentence building & serialization
- [x] Reply sentence parsing
- [x] Command sentence parsing
- [x] Reply sentence builder
- [ ] Stream-oriented parser (to support chunk-based parsing, in case chunks are smaller than total sentence length)
- [ ] More memory efficient parsing & serialization
