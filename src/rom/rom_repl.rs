use std::io;
use std::io::Write;

use crate::rom::ZK::*;

// // pub fn new(input: &str) -> ROMs {
// //     // cli input arguments (clap?)
// //     // <binary> <file descriptor> <file descriptor> ...
// //     // run each sequentially, stop on first error
// //     // <binary> "<string>"
// //     // run as if it were a file
// //     // <binary>
// //     // open repl, parse at each newline,
// //     // repl ignores newlines escaped with '\'
// //     // todo: implement
// //     // todo: flags
// //     // ROMize(input)
// // }

// // fn rep(input: &str) -> ROMs {
// //     new(input)
// // }

pub fn repl() -> ! {
    std::io::stdout().flush().unwrap();
    print!("ROM> ");
    loop {
        let mut input_line = String::new();
        let mut is_line_complete = false;
        let mut last_had_content = 0;

        std::io::stdout().flush().unwrap();

        while !is_line_complete {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed_input = input.trim();
            input_line.push_str(trimmed_input);
            input_line.push_str("\n");
            if input_line.ends_with("\\")
                || (!input_line.ends_with("\\") && trimmed_input == "" && last_had_content < 1)
            {
                last_had_content += 1;
                input_line.push(' ')
            } else if !input_line.ends_with("\\") && trimmed_input == "" {
                is_line_complete = true;
            } else if !input_line.ends_with("\\") {
                last_had_content = 0;
            } else if last_had_content > 5 {
                is_line_complete = true;
            }
        }

        print!(
            "\nROM:\n{:#?}\n\nROM> ",
            Line {
                zk: ZK {
                    text: None,
                    time: "go time".to_string(),
                    time_accuracy: None,
                    time_zone: None,
                    year: "123".to_string(),
                    locale: None,
                    location: None,
                    hash: "hashbrown milkshake".to_string(),
                    tags: None,
                    categories: None,
                    comments: None,
                    signatures: None,
                },
                comments: None,
                citation: None,
                citations: None,
                signatures: None,
            }
        );

        let x = Binder{
            hash: "1".to_string(),
            visibility: Some("2".to_string()),
            private: Some("3".to_string()),
            public: Some(true),
            private: Some(false),
            notes: Some(
                Notes{
                  hash: "4".to_string(),
    // identifier: Identifier { },
    visibility: Some("5".to_string()),
    private: Some(true),
    public: Some(true),
    file: "6".to_string(),
    note: Note {
        hash: "7".to_string(),
        lines: Vec::new().push(
        Line {
            zk: ZK {
                text: None,
                time: "8".to_string(),
                time_accuracy: None,
                time_zone: None,
                year: "9".to_string(),
                locale: None,
                location: None,
                hash: "10".to_string(),
                tags: None,
                categories: None,
                comments: None,
                signatures: None,
            },
            comments: None,
            citation: None,
            citations: None,
            signatures: None,
        })},
    }),
    line: Line {
            zk: ZK {
                text: None,
                time: "8".to_string(),
                time_accuracy: None,
                time_zone: None,
                year: "9".to_string(),
                locale: None,
                location: None,
                hash: "10".to_string(),
                tags: None,
                categories: None,
                comments: None,
                signatures: None,
            },
            comments: None,
            citation: None,
            citations: None,
            signatures: None,
        }},
    notes: Vec<Note>,
    text: "number".to_string(),
    line: Option<Line>,
    lines: Option<Lines>,
    verifications: Option<Verifications>,
    groups: Option<Groups>,
    local_identifier: Option<Identifier>,
    remote_identifier: Option<Identifier>,
    author_data: Option<Vec<Note>>,
    backlinks: Option<Citations>,
    backlinks_identifier: Option<Identifier>,
    cite: Option<Citation>,
    cite_path: "number".to_string(),
    citation: Option<Citation>,
    citation_identifier: Option<Identifier>,
    citations: Option<Citations>,
    citations_identifier: Option<Identifier>,
    description: Option<Description>,
}
            // public: Option<bool>,
            // notes: Notes,
            // binder: Vec<SlipBox>,
            // identifier: Option<Identifier>,
            // ownership: Option<Ownership>,
            // description: Option<Description>,
            // identifiers: Option<Identifiers>,
        };
}
        std::io::stdout().flush().unwrap();
    }
}
