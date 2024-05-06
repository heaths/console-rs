// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use console::{console::Console, Result};
use std::io::Write;

fn main() -> Result<()> {
    let mut console = Console::from_system()?;
    writeln!(console, "Hello, world!")?;
    writeln!(console.stderr(), "error: no listeners")?;

    Ok(())
}
