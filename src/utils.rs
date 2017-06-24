// Copyright (C) 2017+ Sandeep Datta

macro_rules! println_err {
    ($($arg:tt)*) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr (file handle closed?): {}", x),
        }
    })
}

macro_rules! print_err {
    ($($arg:tt)*) => ({
        use std::io::Write;
        match write!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr (file handle closed?): {}", x),
        }
    })
}
