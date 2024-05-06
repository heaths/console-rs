// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![cfg(windows)]
#![allow(clippy::upper_case_acronyms)]

use std::{
    io::IsTerminal,
    os::windows::{io::AsRawHandle, raw::HANDLE},
};

type DWORD = u32;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 4;

#[repr(transparent)]
struct BOOL(i32);

impl BOOL {
    #[inline]
    fn ok(self) -> bool {
        self.0 != 0
    }
}

impl From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.ok()
    }
}

extern "C" {
    fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: &mut DWORD) -> BOOL;
    fn GetLastError() -> DWORD;
    fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
}

pub fn enable_virtual_terminal_processing<T>(fd: &T) -> crate::Result<()>
where
    T: IsTerminal + AsRawHandle,
{
    if !fd.is_terminal() {
        return Ok(());
    }

    let mut mode: DWORD = 0;
    unsafe {
        if GetConsoleMode(fd.as_raw_handle(), &mut mode).ok()
            && SetConsoleMode(
                fd.as_raw_handle(),
                mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            )
            .ok()
        {
            return Ok(());
        }

        Err(GetLastError().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn bool_compatible() {
        let b = BOOL(0);
        assert_eq!(false, b.into());

        let b = BOOL(1);
        assert_eq!(true, b.into());
    }
}
