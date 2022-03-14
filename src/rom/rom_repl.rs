use std::io::Write;
use std::{io, rc::Rc};

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
                // citation: None,
                citations: None,
                signatures: None,
            }
        );
        let zkc = ZK {
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
        };
        let ll = Line {
            zk: zkc,
            comments: None,
            // citation: None,
            citations: None,
            signatures: None,
        };
        let zxs = ZK {
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
        };
        let lz = Line {
            zk: zxs,
            comments: None,
            // citation: None,
            citations: None,
            signatures: None,
        };
        let zb = {
            let mut x = Vec::new();
            x.push(lz);
            Lines {
                lines: x,
                hash: "7".to_string(),
                line: Some(ll),
            }
        };
        let zz = Note {
            hash: "7".to_string(),
            lines: zb,
            identifier: None,
            identifiers: None,
            description: None,
            verifications: None,
        };
        let xx = Vec::new();

        let xxx = {
            // x.push(Note {
            //     hash: "7".to_string(),
            //     lines: {x.push(
            //         let x = Vec::new();
            //         x.push(Line {
            //             zk: ZK {
            //                 text: None,
            //                 time: "8".to_string(),{
            let mut x = xx;
            x.push(Line {
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
                // citation: None,
                citations: None,
                signatures: None,
            });
            Lines {
                lines: x,
                hash: "7".to_string(),
                line: Some(Line {
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
                    // citation: None,
                    citations: None,
                    signatures: None,
                }),
            }
        };
        // //                 time_accuracy: None,
        // //                 time_zone: None,
        // //                 year: "9".to_string(),
        // //                 locale: None,
        // //                 location: None,
        // //                 hash: "10".to_string(),
        // //                 tags: None,
        // //                 categories: None,
        // //                 comments: None,
        // //                 signatures: None,
        // //             },
        // //             comments: None,
        // //             // citation: None,
        // //             citations: None,
        // //             signatures: None,
        // //         });
        // //         Lines {
        // //             lines: x,
        // //             hash: "7".to_string(),
        // //             line: Some(Line {
        // //                 zk: ZK {
        // //                     text: None,
        // //                     time: "8".to_string(),
        // //                     time_accuracy: None,
        // //                     time_zone: None,
        // //                     year: "9".to_string(),
        // //                     locale: None,
        // //                     location: None,
        // //                     hash: "10".to_string(),
        // //                     tags: None,
        // //                     categories: None,
        // //                     comments: None,
        // //                     signatures: None,
        // //                 },
        // //                 comments: None,
        // //                 // citation: None,
        // //                 citations: None,
        // //                 signatures: None,
        // //             }),
        // //         }
        // //     },
        // //     identifier: None,
        // //     identifiers: None,
        // //     description: None,
        // //     verifications: None,
        // // });
        let ni = Identifier {
            name: "test".to_string(),
            aliases: None,
            hash: "1".to_string(),
            hash_aliases: None,
            file: None,
            prefix: None,
            prefix_aliases: None,
            prefix_hash: None,
            prefix_hash_aliases: None,
            root: None,
            remote_root: None,
            remote_tap_root: None,
            path: None,
            url: None,
            tags: None,
            categories: None,
            comments: None,
            identifiers: None,
            backlinks: None,
            references: None,
        };
        let z = Vec::new();
        let nn = Notes {
            hash: "4".to_string(),
            identifier: ni,
            // identifier: Identifier { },
            visibility: Some("5".to_string()),
            private: Some(true),
            public: Some(true),
            file: Some("6".to_string()),
            note: zz,
            notes: z,
            author_data: None,
            backlinks: None,
            backlinks_identifier: None,
            cite: None,
            cite_identifier: None,
            citation_identifier: None,
            citation: None,
            citations: None,
            citations_identifier: None,
            description: None,
            verifications: None,
            text: None,
            line: None,
            lines: None,
            groups: None,
            local_identifier: None,
            remote_identifier: None,
        };
        let ii = Identifier {
            name: "test".to_string(),
            aliases: None,
            hash: "1".to_string(),
            hash_aliases: None,
            file: None,
            prefix: None,
            prefix_aliases: None,
            prefix_hash: None,
            prefix_hash_aliases: None,
            root: None,
            remote_root: None,
            remote_tap_root: None,
            path: None,
            url: None,
            tags: None,
            categories: None,
            comments: None,
            identifiers: None,
            backlinks: None,
            references: None,
        };
        let iii = Identifier {
            name: "test".to_string(),
            aliases: None,
            hash: "1".to_string(),
            hash_aliases: None,
            file: None,
            prefix: None,
            prefix_aliases: None,
            prefix_hash: None,
            prefix_hash_aliases: None,
            root: None,
            remote_root: None,
            remote_tap_root: None,
            path: None,
            url: None,
            tags: None,
            categories: None,
            comments: None,
            identifiers: None,
            backlinks: None,
            references: None,
        };
        // make slipbox
        // let zyx = {
        //     let mut x = Vec::new();
        //     x.push(nn);
        //     Notes {
        //         hash: "4".to_string(),
        //         identifier: iii,
        //         // identifier: Identifier { },
        //         visibility: Some("5".to_string()),
        //         private: Some(true),
        //         public: Some(true),
        //         file: Some("6".to_string()),
        //         note: zz,
        //         notes: z,
        //         author_data: None,
        //         backlinks: None,
        //         backlinks_identifier: None,
        //         cite: None,
        //         cite_identifier: None,
        //         citation_identifier: None,
        //         citation: None,
        //         citations: None,
        //         citations_identifier: None,
        //         description: None,
        //         verifications: None,
        //         text: None,
        //         line: None,
        //         lines: None,
        //         groups: None,
        //         local_identifier: None,
        //         remote_identifier: None,
        //     }
        // };
        let n = {
            let x = Vec::new();
            x
        };
        // make lines

        let l = Line {
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
            // citation: None,
            citations: None,
            signatures: None,
        };
        let l2 = Lines {
            hash: "123".to_string(),
            lines: vec![l],
            line: None,
        };
        // make note
        let nt = Note {
            lines: l2,
            hash: "11".to_string(),
            identifier: None,
            identifiers: None,
            description: None,
            verifications: None,
        };
        // make cs variable which is a Notes object
        let cs = Notes {
            hash: "4".to_string(),
            identifier: ii,
            // identifier: Identifier { },
            visibility: Some("5".to_string()),
            private: Some(true),
            public: Some(true),
            file: Some("6".to_string()),
            note: nt,
            notes: n,
            author_data: None,
            backlinks: None,
            backlinks_identifier: None,
            cite: None,
            cite_identifier: None,
            citation_identifier: None,
            citation: None,
            citations: None,
            citations_identifier: None,
            description: None,
            verifications: None,
            text: None,
            line: None,
            lines: None,
            groups: None,
            local_identifier: None,
            remote_identifier: None,
        };
        let we = {
            let mut x = Vec::new();
            x.push(cs);
            x
        };
        // make identifier
        let id = Identifier {
            name: "test".to_string(),
            aliases: None,
            hash: "1".to_string(),
            hash_aliases: None,
            file: None,
            prefix: None,
            prefix_aliases: None,
            prefix_hash: None,
            prefix_hash_aliases: None,
            root: None,
            remote_root: None,
            remote_tap_root: None,
            path: None,
            url: None,
            tags: None,
            categories: None,
            comments: None,
            identifiers: None,
            backlinks: None,
            references: None,
        };
        // let mut daz = Vec::new();
        let line1 = Line {
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
            // citation: None,
            citations: None,
            signatures: None,
        };
        let lines2 = Lines {
            hash: "123".to_string(),
            lines: vec![line1],
            line: None,
        };
        let asd = Note {
            lines: lines2,
            hash: "11".to_string(),
            identifier: None,
            identifiers: None,
            description: None,
            verifications: None,
        };
        let l3l = Line {
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
            // citation: None,
            citations: None,
            signatures: None,
        };
        let l3 = Lines {
            hash: "123".to_string(),
            lines: vec![l3l],
            line: None,
        };
        // make note
        let nt3 = Note {
            lines: l3,
            hash: "11".to_string(),
            identifier: None,
            identifiers: None,
            description: None,
            verifications: None,
        };
        let notes = Notes {
            hash: "4".to_string(),
            identifier: id,
            visibility: Some("5".to_string()),
            private: Some(true),
            public: Some(true),
            file: Some("6".to_string()),
            note: asd,
            notes: vec![nt3],
            author_data: None,
            backlinks: None,
            backlinks_identifier: None,
            cite: None,
            cite_identifier: None,
            citation_identifier: None,
            citation: None,
            citations: None,
            citations_identifier: None,
            description: None,
            verifications: None,
            text: None,
            line: None,
            lines: None,
            groups: None,
            local_identifier: None,
            remote_identifier: None,
        };
        let ident11 = Identifier {
            name: "test".to_string(),
            aliases: None,
            hash: "1".to_string(),
            hash_aliases: None,
            file: None,
            prefix: None,
            prefix_aliases: None,
            prefix_hash: None,
            prefix_hash_aliases: None,
            root: None,
            remote_root: None,
            remote_tap_root: None,
            path: None,
            url: None,
            tags: None,
            categories: None,
            comments: None,
            identifiers: None,
            backlinks: None,
            references: None,
        };
        let idddd = Identifier {
            name: "test".to_string(),
            aliases: None,
            hash: "1".to_string(),
            hash_aliases: None,
            file: None,
            prefix: None,
            prefix_aliases: None,
            prefix_hash: None,
            prefix_hash_aliases: None,
            root: None,
            remote_root: None,
            remote_tap_root: None,
            path: None,
            url: None,
            tags: None,
            categories: None,
            comments: None,
            identifiers: None,
            backlinks: None,
            references: None,
        };

        let author1 = Author {
            identifier: idddd,
            hash: "asd".to_string(),
            description: None,
            identifiers: None,
            bio: None,
            notes: None,
        };
        let own = Ownership {
            identifier: ident11,
            identifiers: None,
            description: None,
            author: Some(author1),
            authors: None,
            group: None,
            groups: None,
        };
        let m = SlipBox {
            notes: notes,
            identifier: None,
            hash: "1".to_string(),
            slipbox: we,
            visibility: None,
            private: None,
            public: None,
            ownership: Some(own),
        };
        let mut yyy = Vec::new();
        yyy.push(m);
        let y = Binder {
            hash: "1".to_string(),
            visibility: Some("2".to_string()),
            public: Some(true),
            private: Some("yeah".to_ascii_lowercase()),
            identifier: Some(iii),
            notes: nn,
            ownership: None,
            binder: yyy,
        };
        // let x = nn;
        let x //= m;
        = Box::<Binder>::new(y);

        println!("${x:#?}");
    }
}

//                 Notes {
//                     notes: x,
//                     hash: "7".to_string(),
//                     note: Note {
//                         hash: "7".to_string(),
//                         lines: {
//                             let x = Vec::new();
//                             x.push(
//                                 Line {
//                                     zk: ZK {
//                                         text: None,
//                                         time: "8".to_string(),
//                                         time_accuracy: None,
//                                         time_zone: None,
//                                         year: "9".to_string(),
//                                         locale: None,
//                                         location: None,
//                                         hash: "10".to_string(),
//                                         tags: None,
//                                         categories: None,
//                                         comments: None,
//                                         signatures: None,
//                                     },
//                                     comments: None,
//                                     citation: None,
//                                     citations: None,
//                                     signatures: None,
//                                 });
//                                 Lines {
//                                     lines: x,
//                                     hash: "7".to_string(),
//                                     line: Some(Line {
//                                         zk: ZK {
//                                             text: None,
//                                             time: "8".to_string(),
//                                             time_accuracy: None,
//                                             time_zone: None,
//                                             year: "9".to_string(),
//                                             locale: None,
//                                             location: None,
//                                             hash: "10".to_string(),
//                                             tags: None,
//                                             categories: None,
//                                             comments: None,
//                                             signatures: None,
//                                         },
//                                         comments: None,
//                                         citation: None,
//                                         citations: None,
//                                         signatures: None,
//                                     })
//                                 }
//                         },
//                         identifier: None,
//                         identifiers: None,
//                         description: None,
//                         verifications: None,
//                     },
// identifier: None,
// description: None,
// verifications: None,
// visibility: Some("5".to_string()),
// private: Some(true),
// public: Some(true),
// file: Some("6".to_string()),
// text: None,
// line: None,
// lines: None,
// groups: None,
// local_identifier: None,
// remote_identifier: None,
//                     author_data: None,
//                     // author_data_hash: None,
//                     // need a generic hash vs specific hash -- my note about "black holes" is in the set the subject "black holes" either because category=black\ holes or maybe topic is /sci/black\ holes/ or really /**/black\ holes/ or something
//                     backlinks: None,
//                     backlinks_identifier: None,
// cite: None,
// cite_identifier: None,
// citation_identifier: None,
// citation: None,
// citations: None,
// citations_identifier: None,
//                 }

//     text: "number".to_string(),
//     line: Line {
//             zk: ZK {
//                 text: None,
//                 time: "8".to_string(),
//                 time_accuracy: None,
//                 time_zone: None,
//                 year: "9".to_string(),
//                 locale: None,
//                 location: None,
//                 hash: "10".to_string(),
//                 tags: None,
//                 categories: None,
//                 comments: None,
//                 signatures: None,
//             },
//             comments: None,
//             citation: None,
//             citations: None,
//             signatures: None,
//     },
//     lines: Vec::new().push(
//         Line {
//             zk: ZK {
//                 text: None,
//                 time: "8".to_string(),
//                 time_accuracy: None,
//                 time_zone: None,
//                 year: "9".to_string(),
//                 locale: None,
//                 location: None,
//                 hash: "10".to_string(),
//                 tags: None,
//                 categories: None,
//                 comments: None,
//                 signatures: None,
//             },
//             comments: None,
//             citation: None,
//             citations: None,
//             signatures: None,
//         }
//         ),
//     verifications: None,
//     groups: None,
//     local_identifier: None,
//     remote_identifier: None,
//     author_data: None,
//     backlinks: None,
//     backlinks_identifier: None,
//     cite: None,
//     cite_identifier: "number".to_string(),
//     citation: None,
//     citation_identifier: None,
//     citations: None,
//     citations_identifier: None,
//     description: None,
//     identifier: None,
//     }
// };
