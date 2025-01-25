# global-channel

`global-channel` is a lightweight Rust crate that provides a simple way to define and use global, static channels for message passing. It supports both bounded and unbounded channels, powered by `crossbeam-channel`.


## Features

- Define static, global channels with minimal boilerplate.
- Bounded and unbounded channels.
- Safe initialization using `std::sync::Once`.
- Highly performant with `crossbeam-channel`.

## Usage

Create and use a global channel with the `global_channel!` macro:

```rust
use global_channel::global_channel;

global_channel!(my_channel, Some(10), u32); // Bounded channel with capacity 10

fn main() {
    let tx = my_channel_tx();
    let rx = my_channel_rx();

    tx.send(42).unwrap();
    println!("Received: {}", rx.recv().unwrap());
}
```

For an unbounded channel:

```rust
global_channel!(unbounded_channel, None, String);
```


## Basic Example

```rust
use global_channel::global_channel;

global_channel!(example_channel, None, i32);

fn main() {
    let tx = example_channel_tx();
    let rx = example_channel_rx();

    tx.send(100).unwrap();
    assert_eq!(rx.recv().unwrap(), 100);
}
```

---

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-Apache-2.0](LICENSE-Apache-2.0) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

