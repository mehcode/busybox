use std::{
    borrow::Cow,
    env::args,
    fs::File,
    io::{self, BufWriter, Write},
};

fn main() -> io::Result<()> {
    // Take the second argument if present; else, use "y".

    // Cow is an enum that allows to represent either an owned or borrowed value in the
    //  same type. We use Cow here as the argument iterator returns a `String` but
    //  our default value is a statically borrowed `str` literal.

    // We could remove Cow and do:
    //  args().nth(1).unwrap_or("y".to_owned())

    // However, that would result in, potentially, an unneccessary allocation.

    let expletive = args().nth(1).map(|mut s| {
        // Append a newline to allow for 1 write call later instead of 2
        s.push('\n');

        Cow::Owned(s)
    }).unwrap_or(Cow::Borrowed("y\n"));

    // In Rust, stdout is _always_ line-buffered. This works for most cases of simple
    // printing. However, we want to get raw access to stdout and print large amounts
    // of text at once. To do that, we need to drop down and use unsafe C APIs to get stdout
    // directly and wrap in a block buffered writer.

    let out = stdout();
    let mut buf = BufWriter::new(out);

    loop {
        match buf.write_all(expletive.as_bytes()) {
            // If the operation was successful, continue to loop
            Ok(()) => {}

            // A BrokenPipe error indicates that the commmand we are
            // connected to closed. We should now cleanly exit.
            Err(ref err) if err.kind() == io::ErrorKind::BrokenPipe => {
                return Ok(());
            }

            // Return other errors from main which should exit with a failure
            // status and print the error.
            Err(err) => {
                return Err(err);
            }
        }
    }

    // No need to return. The compiler understands the above loop is infinite.
}

#[cfg(unix)]
fn stdout() -> File {
    use libc::STDOUT_FILENO;
    use std::os::unix::io::FromRawFd;

    unsafe { File::from_raw_fd(STDOUT_FILENO) }
}

#[cfg(windows)]
fn stdout() -> File {
    use std::os::windows::io::FromRawHandle;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;

    let handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    unsafe { File::from_raw_handle(handle) }
}
