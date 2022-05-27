fn impossible_operation;

fn zero_division;

fn not_a_number;

fn unknown_error;

fn less_than_reqd_args;

fn unknown_flag;


/*

If you want your own data types to be printable for debugging and logging, 
you can in most cases add a #[derive(Debug)] above their definition.

Aside: “User-friendly” printing is done using the Display trait,
debug output (human-readable but targeted at developers) uses the Debug trait. 
You can find more information about the syntax you can use in println! in the documentation for the std::fmt module.

* Error output

Printing errors should be done via stderr to make it easier for users and other tools to pipe their outputs to files or more tools.

Aside: On most operating systems, a program can write to two output streams, stdout and stderr. stdout is for the program’s actual output, 
while stderr allows errors and other messages to be kept separate from stdout. That way, output can be stored to a file or piped to another program while errors are shown to the user.

* In Rust 
this is achieved with println! and eprintln!, 
the former printing to stdout and the latter to stderr.

Beware: Printing escape codes can be dangerous, 
putting the user’s terminal into a weird state. 
Always be careful when manually printing them!

Ideally you should be using a crate like ansi_term when dealing with raw escape codes to 
make your (and your user’s) life easier.

* A note on printing performance

Printing to the terminal is surprisingly slow! 
If you call things like println! in a loop, 
it can easily become a bottleneck in an otherwise fast program. 
To speed this up, there are two things you can do.

First, you might want to reduce the number of writes that actually “flush” to the terminal. 
println! tells the system to flush to the terminal every time,
because it is common to print each new line. 
If you don’t need that, you can wrap your stdout handle in a BufWriter which by default buffers up to 8 kB. (
You can still call .flush() on this BufWriter when you want to print immediately.)

use std::io::{self, Write};

let stdout = io::stdout(); // get the global stdout entity
let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

use std::io::{self, Write};

let stdout = io::stdout(); // get the global stdout entity
let mut handle = stdout.lock(); // acquire a lock on it
writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
*/