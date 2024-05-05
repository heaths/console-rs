// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::{
    io::Read,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default)]
pub struct Buffer<'a> {
    buffer: Either<'a, Vec<u8>>,
    tty: bool,
}

impl<'a> Buffer<'a> {
    pub fn from<T>(buffer: Option<&'a mut T>, tty: bool) -> Self
    where
        T: AsMut<Vec<u8>> + 'a,
    {
        Self {
            buffer: buffer.map_or_else(
                || Either::Owned(Vec::new()),
                |buf| Either::Borrowed(buf.as_mut()),
            ),
            tty,
        }
    }

    pub fn tty(&self) -> bool {
        self.tty
    }

    #[cfg(test)]
    fn is_owned(&self) -> bool {
        if let Either::Owned(_) = &self.buffer {
            return true;
        }
        false
    }
}

impl<'a> Deref for Buffer<'a> {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        match &self.buffer {
            Either::Owned(v) => v,
            Either::Borrowed(ref v) => v,
        }
    }
}

impl<'a> DerefMut for Buffer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.buffer {
            Either::Owned(v) => v,
            Either::Borrowed(ref mut v) => v,
        }
    }
}

impl<'a> Read for Buffer<'a> {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn read_vectored(&mut self, _bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        todo!()
    }
}

#[derive(Debug)]
enum Either<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
}

impl<'a, T> Default for Either<'a, T>
where
    T: Default,
{
    fn default() -> Self {
        Either::Owned(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn owned() {
        let mut buffer = Buffer::default();
        assert!(buffer.is_owned());
        assert!(!buffer.tty);

        write!(buffer, "test").unwrap();
        assert_eq!("test".as_bytes(), (*buffer).as_slice());
    }

    #[test]
    fn borrowed() {
        let mut bytes: Vec<u8> = Vec::new();
        let mut buffer = Buffer::from(Some(&mut bytes), true);
        assert!(!buffer.is_owned());
        assert!(buffer.tty);

        write!(buffer, "test").unwrap();
        assert_eq!("test".as_bytes(), bytes.as_slice());
    }
}
