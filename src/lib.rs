// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::{
    io::{self, Read, Write},
    ops::DerefMut,
};

mod buffer;
mod error;
#[cfg(windows)]
mod windows;

pub use buffer::*;
pub use error::*;

/// Represents a console, wrapping [`io::Stdin`], [`io::Stdout`], and [`io::Stderr`] for production or testing.
#[derive(Debug)]
pub struct Console<'a> {
    buffers: Buffers<'a>,
}

impl<'a> Console<'a> {
    /// Gets a [`Console`] that uses system file descriptors e.g., from [`std::io::stdout()`].
    pub fn from_system() -> Result<Self> {
        let stdout = io::stdout();
        let stderr = io::stderr();

        #[cfg(windows)]
        {
            windows::enable_virtual_terminal_processing(stdout)?;
            windows::enable_virtual_terminal_processing(stderr)?;
        }

        Ok(Self {
            buffers: Buffers::System {
                stdin: io::stdin(),
                stdout: io::stdout(),
                stderr: io::stderr(),
            },
        })
    }

    /// Build a [`Console`] from optional buffers and whether each buffer mimics a TTY.
    pub fn builder<'b>() -> builders::ConsoleBuilder<'b> {
        builders::ConsoleBuilder::default()
    }

    /// Gets a representation of stdin.
    pub fn stdin(&'a mut self) -> Stdin<'a> {
        match &mut self.buffers {
            Buffers::System { stdin, .. } => Stdin { buffer: stdin },
            Buffers::Internal { stdin, .. } => Stdin { buffer: stdin },
        }
    }

    /// Gets a representation of stdout.
    pub fn stdout(&'a mut self) -> Stdout<'a> {
        match &mut self.buffers {
            Buffers::System { stdout, .. } => Stdout { buffer: stdout },
            Buffers::Internal { stdout, .. } => Stdout {
                buffer: stdout.deref_mut(),
            },
        }
    }

    /// Gets a representation of stderr.
    pub fn stderr(&'a mut self) -> Stderr<'a> {
        match &mut self.buffers {
            Buffers::System { stderr, .. } => Stderr { buffer: stderr },
            Buffers::Internal { stderr, .. } => Stderr {
                buffer: stderr.deref_mut(),
            },
        }
    }
}

impl<'a> Default for Console<'a> {
    /// Gets a [`Console`] from [`Console::from_system()`].
    fn default() -> Self {
        let stdout = io::stdout();
        let stderr = io::stderr();

        #[cfg(windows)]
        {
            // Attempt to enable virtual processing but do not panic otherwise.
            let _ = windows::enable_virtual_terminal_processing(stdout);
            let _ = windows::enable_virtual_terminal_processing(stderr);
        }

        Self {
            buffers: Buffers::System {
                stdin: io::stdin(),
                stdout: io::stdout(),
                stderr: io::stderr(),
            },
        }
    }
}

impl<'a> Write for Console<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.buffers {
            Buffers::System { stdout, .. } => stdout.write(buf),
            Buffers::Internal { stdout, .. } => stdout.write(buf),
        }
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        match &mut self.buffers {
            Buffers::System { stdout, .. } => stdout.write_vectored(bufs),
            Buffers::Internal { stdout, .. } => stdout.write_vectored(bufs),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match &mut self.buffers {
            Buffers::System { stdout, .. } => stdout.flush(),
            Buffers::Internal { stdout, .. } => stdout.flush(),
        }
    }
}

pub mod builders {
    use super::*;

    /// Builds a [`Console`] from optional buffers and whether each buffer mimics a TTY.
    #[must_use]
    #[derive(Debug, Default)]
    pub struct ConsoleBuilder<'a> {
        stdin: Option<&'a mut Vec<u8>>,
        stdin_tty: bool,
        stdout: Option<&'a mut Vec<u8>>,
        stdout_tty: bool,
        stderr: Option<&'a mut Vec<u8>>,
        stderr_tty: bool,
    }

    impl<'a> ConsoleBuilder<'a> {
        /// A buffer to use for stdin instead of an empty buffer.
        pub fn stdin(mut self, stdin: &'a mut Vec<u8>) -> Self {
            self.stdin = Some(stdin);
            self
        }

        pub fn stdin_tty(mut self, tty: bool) -> Self {
            self.stdin_tty = tty;
            self
        }

        /// A buffer to use for stdout instead of an empty buffer.
        pub fn stdout(mut self, stdout: &'a mut Vec<u8>) -> Self {
            self.stdout = Some(stdout);
            self
        }

        pub fn stdout_tty(mut self, tty: bool) -> Self {
            self.stdout_tty = tty;
            self
        }

        /// A buffer to use for stdin instead of an empty buffer.
        pub fn stderr(mut self, stderr: &'a mut Vec<u8>) -> Self {
            self.stderr = Some(stderr);
            self
        }

        pub fn stderr_tty(mut self, tty: bool) -> Self {
            self.stderr_tty = tty;
            self
        }

        pub fn build(self) -> Console<'a> {
            Console {
                buffers: Buffers::Internal {
                    stdin: Buffer::from(self.stdin, self.stdin_tty),
                    stdout: Buffer::from(self.stdout, self.stdout_tty),
                    stderr: Buffer::from(self.stderr, self.stderr_tty),
                },
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Buffers<'a> {
    System {
        stdin: io::Stdin,
        stdout: io::Stdout,
        stderr: io::Stderr,
    },
    Internal {
        stdin: Buffer<'a>,
        stdout: Buffer<'a>,
        stderr: Buffer<'a>,
    },
}

/// Represents stdin.
pub struct Stdin<'a> {
    buffer: &'a mut dyn Read,
}

impl<'a> Read for Stdin<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buffer.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.buffer.read_vectored(bufs)
    }
}

/// Represents stdout.
pub struct Stdout<'a> {
    buffer: &'a mut dyn Write,
}

impl<'a> Write for Stdout<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.buffer.write_vectored(bufs)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

/// Represents stderr.
pub struct Stderr<'a> {
    buffer: &'a mut dyn Write,
}

impl<'a> Write for Stderr<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.buffer.write_vectored(bufs)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_outs() {
        let mut stdout: Vec<u8> = Vec::new();
        let mut stderr: Vec<u8> = Vec::new();

        let mut console = Console::builder()
            .stdout(&mut stdout)
            .stderr(&mut stderr)
            .build();
        writeln!(console, "Hello, world!").unwrap();
        writeln!(console.stderr(), "error: no listeners").unwrap();

        assert_eq!("Hello, world!\n".as_bytes(), stdout.as_slice());
        assert_eq!("error: no listeners\n".as_bytes(), stderr.as_slice());
    }
}
