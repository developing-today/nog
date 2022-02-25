use std::io;
use std::io::Write;

use chiploxide::rom::ZK::Line;



// pub fn new(input: &str) -> ROMs {
//     // cli input arguments (clap?)
//     // <binary> <file descriptor> <file descriptor> ...
//     // run each sequentially, stop on first error
//     // <binary> "<string>"
//     // run as if it were a file
//     // <binary>
//     // open repl, parse at each newline,
//     // repl ignores newlines escaped with '\'
//     // todo: implement
//     // todo: flags
//     // ROMize(input)
// }

// fn rep(input: &str) -> ROMs {
//     new(input)
// }

pub fn repl() -> ! {
    std::io::stdout().flush().unwrap();
    print!("ROM> ");
    loop {
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        std::io::stdout().flush().unwrap();
        print!("\nROM:\n{:#?}\n\nROM> ", Line{text: input});
        std::io::stdout().flush().unwrap();
    }
}
