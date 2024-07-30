use anstyle::{AnsiColor, Style};
use anyhow::{Error, Result};
use lazy_static::lazy_static;
use std::fmt::Display;
use std::io::prelude::*;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

lazy_static! {
    pub static ref GlobalShell: Arc<RwLock<Shell>> = Arc::new(RwLock::new(Shell::new()));
}

/// Usage:
///
/// ```rs
/// use npmpink_tui::shell::shell;
/// shell()?.print_err(...)?;
/// ```
pub fn shell<'a>() -> Result<RwLockWriteGuard<'a, Shell>> {
    GlobalShell
        .write()
        .map_err(|err| Error::msg(err.to_string()))
}

#[derive(Debug)]
pub struct Shell {
    out: ShellOut,
}

impl Default for Shell {
    fn default() -> Self {
        Shell::new()
    }
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            out: ShellOut {
                stdout: std::io::stdout(),
                stderr: std::io::stderr(),
            },
        }
    }
    pub fn error<T: Display>(&mut self, msg: T) -> Result<()> {
        self.out.write_stderr(&"ERROR", &msg, None)?;
        Ok(())
    }
    pub fn info<T: Display>(&mut self, msg: T) -> Result<()> {
        self.out.write_stdout(
            &"INFO",
            &msg,
            Some(Style::new().bold().fg_color(Some(AnsiColor::Green.into()))),
            None,
        )?;
        Ok(())
    }
    pub fn warn<T: Display>(&mut self, msg: T) -> Result<()> {
        self.out.write_stdout(
            &"WARN",
            &msg,
            Some(Style::new().bold().fg_color(Some(AnsiColor::Yellow.into()))),
            None,
        )?;
        Ok(())
    }
}

#[derive(Debug)]
struct ShellOut {
    stdout: std::io::Stdout,
    stderr: std::io::Stderr,
}

impl ShellOut {
    pub fn write_stderr(
        &mut self,
        prefix: &dyn Display,
        msg: &dyn Display,
        style: Option<Style>,
    ) -> Result<()> {
        let bold = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
        let style = style.unwrap_or_else(Style::new);

        let mut buffer = Vec::new();

        // prefix
        write!(&mut buffer, "{bold}{prefix}:{bold:#}{style} ")?;
        // message with format
        write!(&mut buffer, "{msg}{style:#}")?;

        self.stderr.write_all(&buffer)?;

        Ok(())
    }

    pub fn write_stdout(
        &mut self,
        prefix: &dyn Display,
        msg: &dyn Display,
        prefix_style: Option<Style>,
        style: Option<Style>,
    ) -> Result<()> {
        let mut buffer = Vec::new();
        let prefix_style = prefix_style.unwrap_or_default();
        let style = style.unwrap_or_default();

        write!(
            &mut buffer,
            "{prefix_style}{prefix}:{prefix_style:#}{style} "
        )?;
        write!(&mut buffer, "{msg}{style:#}")?;

        self.stdout.write_all(&buffer)?;

        Ok(())
    }
}
