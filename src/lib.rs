// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![doc = include_str!("../README.md")]

#[cfg(feature = "console")]
pub mod console;
mod error;
#[cfg(windows)]
mod windows;

pub use error::*;
use std::io::IsTerminal;

#[cfg(not(windows))]
pub fn enable_ansi_escape_sequences<T>(fd: T) -> Result<()>
where
    T: IsTerminal,
{
    Ok(())
}

#[cfg(windows)]
pub fn enable_ansi_escape_sequences<T>(fd: T) -> Result<()>
where
    T: IsTerminal + std::os::windows::io::AsRawHandle,
{
    windows::enable_virtual_terminal_processing(fd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn can_enable_ansi_escape_sequences() {
        // This is unlikely to fail but here to test the definition across platforms.
        enable_ansi_escape_sequences(io::stdout()).unwrap();
    }
}
