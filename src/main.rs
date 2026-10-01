use lexopt::Parser;
use lexopt::prelude::*;
use std::fs;

struct Arguments {
    filename: Option<String>,
    lines: bool,
    bytes: bool,
    words: bool,
}

fn parse_args() -> Result<Arguments, lexopt::Error> {
    let mut parser = Parser::from_env();
    let mut arguments = Arguments {
        filename: None,
        lines: false,
        bytes: false,
        words: false,
    };

    while let Some(arg) = parser.next()? {
        match arg {
            Short('l') | Long("lines") => arguments.lines = true,
            Short('c') | Long("bytes") => arguments.bytes = true,
            Short('w') | Long("words") => arguments.words = true,
            Value(val) => {
                arguments.filename = Some(val.string()?);
            }
            _ => return Err(arg.unexpected().into()),
        }
    }

    Ok(arguments)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;

    let filename = args.filename.ok_or("No file specified")?;
    let contents = fs::read_to_string(&filename)?;

    let mut show_lines = args.lines;
    let mut show_words = args.words;
    let mut show_bytes = args.bytes;

    if !show_lines && !show_words && !show_bytes {
        show_lines = true;
        show_words = true;
        show_bytes = true;
    }

    let mut output = String::new();
    if show_lines {
        output.push_str(&format!(" {} ", contents.lines().count()));
    }
    if show_words {
        output.push_str(&format!(" {} ", contents.split_whitespace().count()));
    }
    if show_bytes {
        output.push_str(&format!(" {} ", contents.len()));
    }
    println!("{}{}", output, filename);

    Ok(())
}
