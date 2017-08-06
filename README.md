# rust-sysbar

Library for interacting with the system's taskbar / tray / statusbar

## Example

```rust
let mut bar = sysbar::Sysbar::new("Foo");

bar.add_item(
    "Say 'bar'",
    Box::new(move || {
        println!("bar");
    }),
);

bar.add_quit_item("Quit");

bar.display();
```

![Resulting screenshot of code above](http://i.imgur.com/mEI6Mxy.png)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
