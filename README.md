# Rust Console APIs

[![ci](https://github.com/heaths/console-rs/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/heaths/console-rs/actions/workflows/ci.yml)

These Rust Console APIs provide a level of abstraction over virtual terminals, implement testable 'std::io::Write'
and will support color schemes.

_This name not final, but aligns with my [github.com/heaths/go-console](https://github.com/heaths/go-console) Go APIs._

## Examples

For all supported versions of Windows, you can enable [ANSI escape sequences](https://learn.microsoft.com/windows/console/console-virtual-terminal-sequences).
This should be unnecessary for other operating systems but the function is conveniently defined for all platforms.

```rust
console::enable_ansi_escape_sequences(std::io::stdout()).unwrap();
println!("\x1b[38;5;128mHello, world!\x1b[m");
```

You can also use the `Console` to write code for production and test:

```rust
use console::console::Console;

fn print(mut w: impl std::io::Write, s: &str) {
    let _ = writeln!(w, "printing: {s}");
}

fn main() {
    let mut console = Console::from_system().unwrap();
    print(console, "sales receipt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn prints_prefix() {
        let mut stdout: Vec<u8> = Vec::new();
        let mut console = Console::builder().stdout(&mut stdout).build();
        print(console, "test");

        assert_eq!("printing: test\n".as_bytes(), stdout.as_slice());
    }
}
```

## License

Licensed under the [MIT](LICENSE.txt) license.
