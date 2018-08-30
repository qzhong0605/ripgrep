extern crate atty;
extern crate same_file;
extern crate termcolor;
#[cfg(windows)]
extern crate winapi_util;

mod wtr;

pub use wtr::{
    StandardStream,
    stdout, stdout_buffered_line, stdout_buffered_block,
};

/// Returns true if and only if stdin is believed to be readable.
///
/// When stdin is readable, command line programs may choose to behave
/// differently than when stdin is not readable. For example, `command foo`
/// might search the current directory for occurrences of `foo` where as
/// `command foo < some-file` or `cat some-file | command foo` might instead
/// only search stdin for occurrences of `foo`.
pub fn is_readable_stdin() -> bool {
    #[cfg(unix)]
    fn imp() -> bool {
        use std::os::unix::fs::FileTypeExt;
        use same_file::Handle;

        let ft = match Handle::stdin().and_then(|h| h.as_file().metadata()) {
            Err(_) => return false,
            Ok(md) => md.file_type(),
        };
        ft.is_file() || ft.is_fifo()
    }

    #[cfg(windows)]
    fn imp() -> bool {
        use winapi_util as winutil;

        winutil::file::typ(winutil::HandleRef::stdin())
            .map(|t| t.is_disk() || t.is_pipe())
            .unwrap_or(false)
    }

    !is_tty_stdin() && imp()
}

/// Returns true if and only if stdin is believed to be connectted to a tty
/// or a console.
pub fn is_tty_stdin() -> bool {
    atty::is(atty::Stream::Stdin)
}

/// Returns true if and only if stdout is believed to be connectted to a tty
/// or a console.
///
/// This is useful for when you want your command line program to produce
/// different output depending on whether it's printing directly to a user's
/// terminal or whether it's being redirected somewhere else. For example,
/// implementations of `ls` will often show one item per line when stdout is
/// redirected, but will condensed output when printing to a tty.
pub fn is_tty_stdout() -> bool {
    atty::is(atty::Stream::Stdout)
}

/// Returns true if and only if stderr is believed to be connectted to a tty
/// or a console.
pub fn is_tty_stderr() -> bool {
    atty::is(atty::Stream::Stderr)
}
