# wagashi

A terminal charting library.

```terminal
cargo add wagashi
```
# Examples

*Sparklines.*

<img width="127" alt="Screen Shot 2022-07-30 at 7 36 57 PM" src="https://user-images.githubusercontent.com/294042/182007265-e13e59c7-adef-4a24-aefb-d43fa8282fc4.png">

```rust
use wagashi::SparkLine;

fn main() {
    let sparkline = SparkLine::new(&[1.0, 2.0, 9.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    sparkline.render();
}
```

# Art

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `wagashi` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
