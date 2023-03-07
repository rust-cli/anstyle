/// Prints to [`stdout`][crate::stdout].
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// Note that stdout is frequently line-buffered by default so it may be
/// necessary to use [`std::io::Write::flush()`] to ensure the output is emitted
/// immediately.
///
/// **NOTE:** The `print!` macro will lock the standard output on each call. If you call
/// `print!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`Stream::lock`][crate::Stream::lock]:
/// ```
/// #  #[cfg(feature = "auto")] {
/// use std::io::Write as _;
///
/// let mut lock = anstyle_stream::stdout().lock();
/// write!(lock, "hello world").unwrap();
/// # }
/// ```
///
/// Use `print!` only for the primary output of your program. Use
/// [`eprint!`] instead to print error and progress messages.
///
/// # Panics
///
/// Panics if writing to `stdout` fails for any reason **except** broken pipe.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// #  #[cfg(feature = "auto")] {
/// use std::io::Write as _;
/// use anstyle_stream::print;
/// use anstyle_stream::stdout;
///
/// print!("this ");
/// print!("will ");
/// print!("be ");
/// print!("on ");
/// print!("the ");
/// print!("same ");
/// print!("line ");
///
/// stdout().flush().unwrap();
///
/// print!("this string has a newline, why not choose println! instead?\n");
///
/// stdout().flush().unwrap();
/// # }
/// ```
#[cfg(feature = "auto")]
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use std::io::Write as _;

        let mut stream = $crate::stdout();
        match ::std::write!(&mut stream, $($arg)*) {
            Err(e) if e.kind() != ::std::io::ErrorKind::BrokenPipe => {
                ::std::panic!("failed printing to stdout: {e}");
            }
            Err(_) | Ok(_) => {}
        }
    }};
}

/// Prints to [`stdout`][crate::stdout], with a newline.
///
/// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`)).
///
/// This macro uses the same syntax as [`format!`], but writes to the standard output instead.
/// See [`std::fmt`] for more information.
///
/// **NOTE:** The `println!` macro will lock the standard output on each call. If you call
/// `println!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`Stream::lock`][crate::Stream::lock]:
/// ```
/// #  #[cfg(feature = "auto")] {
/// use std::io::Write as _;
///
/// let mut lock = anstyle_stream::stdout().lock();
/// writeln!(lock, "hello world").unwrap();
/// # }
/// ```
///
/// Use `println!` only for the primary output of your program. Use
/// [`eprintln!`] instead to print error and progress messages.
///
/// # Panics
///
/// Panics if writing to `stdout` fails for any reason **except** broken pipe.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// #  #[cfg(feature = "auto")] {
/// use anstyle_stream::println;
///
/// println!(); // prints just a newline
/// println!("hello there!");
/// println!("format {} arguments", "some");
/// let local_variable = "some";
/// println!("format {local_variable} arguments");
/// # }
/// ```
#[cfg(feature = "auto")]
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        use std::io::Write as _;

        let mut stream = $crate::stdout();
        match ::std::writeln!(&mut stream, $($arg)*) {
            Err(e) if e.kind() != ::std::io::ErrorKind::BrokenPipe => {
                ::std::panic!("failed printing to stdout: {e}");
            }
            Err(_) | Ok(_) => {}
        }
    }};
}

/// Prints to [`stderr`][crate::stderr].
///
/// Equivalent to the [`print!`] macro, except that output goes to
/// `stderr` instead of `stdout`. See [`print!`] for
/// example usage.
///
/// Use `eprint!` only for error and progress messages. Use `print!`
/// instead for the primary output of your program.
///
/// # Panics
///
/// Panics if writing to `stderr` fails for any reason **except** broken pipe.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// #  #[cfg(feature = "auto")] {
/// use anstyle_stream::eprint;
///
/// eprint!("Error: Could not complete task");
/// # }
/// ```
#[cfg(feature = "auto")]
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        use std::io::Write as _;

        let mut stream = $crate::stderr();
        match ::std::write!(&mut stream, $($arg)*) {
            Err(e) if e.kind() != ::std::io::ErrorKind::BrokenPipe => {
                ::std::panic!("failed printing to stdout: {e}");
            }
            Err(_) | Ok(_) => {}
        }
    }};
}

/// Prints to [`stderr`][crate::stderr], with a newline.
///
/// Equivalent to the [`println!`] macro, except that output goes to
/// `stderr` instead of `stdout`. See [`println!`] for
/// example usage.
///
/// Use `eprintln!` only for error and progress messages. Use `println!`
/// instead for the primary output of your program.
///
/// # Panics
///
/// Panics if writing to `stderr` fails for any reason **except** broken pipe.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// #  #[cfg(feature = "auto")] {
/// use anstyle_stream::eprintln;
///
/// eprintln!("Error: Could not complete task");
/// # }
/// ```
#[cfg(feature = "auto")]
#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::eprint!("\n")
    };
    ($($arg:tt)*) => {{
        use std::io::Write as _;

        let mut stream = $crate::stderr();
        match ::std::writeln!(&mut stream, $($arg)*) {
            Err(e) if e.kind() != ::std::io::ErrorKind::BrokenPipe => {
                ::std::panic!("failed printing to stdout: {e}");
            }
            Err(_) | Ok(_) => {}
        }
    }};
}
