// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use console::{Console, Result};
use std::io::Write;

fn main() -> Result<()> {
    let mut stdout: Vec<u8> = Vec::new();
    let mut stderr: Vec<u8> = Vec::new();

    let mut console = Console::builder()
        .stdout(&mut stdout)
        .stderr(&mut stderr)
        .build();
    writeln!(console, "Hello, world!")?;
    writeln!(console.stderr(), "error: no listeners")?;

    assert_eq!("Hello, world!\n".as_bytes(), stdout.as_slice());
    assert_eq!("error: no listeners\n".as_bytes(), stderr.as_slice());

    print!("{}", String::from_utf8(stdout)?);
    print!("{}", String::from_utf8(stderr)?);

    Ok(())
}
