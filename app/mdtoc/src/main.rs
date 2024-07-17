use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::process::{ExitCode, Termination};

const VERSION: &'static str = "1.0.0";
const USAGE: &'static str = r#"mdtoc 1.0.0: Markdown TOC tool.

Usage: mdtoc [option]... [argument]...

Options and arguments:
        -f, --file <file>       Read markdown content from file.
        -fc, --from-clip        Read markdown content from clipboard.
        -t, --toc               Generate TOC.
        -o, --title-order       Insert order number to title.
        -i, --insert            Insert TOC into the header of the markdown.
        -w, --write <file>      Write result to file.
        -tc, --to-clip          Write result to clipboard.
        -h, --help              Show this help.
        -v, --version           Show version."#;

enum ErrKind {
    InvalidParam(String),
    IoErr(String),
    ClipboardErr(String),
}

impl Display for ErrKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrKind::InvalidParam(msg) => write!(f, "Invalid Param: {}", msg),
            ErrKind::IoErr(msg) => write!(f, "IO Error: {}", msg),
            ErrKind::ClipboardErr(msg) => write!(f, "Clipboard Error: {}", msg),
        }
    }
}

impl Debug for ErrKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for ErrKind {}

impl Termination for ErrKind {
    fn report(self) -> ExitCode {
        match self {
            ErrKind::InvalidParam(_) => ExitCode::from(1),
            ErrKind::IoErr(_) => ExitCode::from(2),
            ErrKind::ClipboardErr(_) => ExitCode::from(3),
        }
    }
}

impl From<std::io::Error> for ErrKind {
    fn from(value: std::io::Error) -> Self {
        ErrKind::IoErr(format!("{value}"))
    }
}
impl From<clipboard_win::ErrorCode> for ErrKind {
    fn from(value: clipboard_win::ErrorCode) -> Self {
        ErrKind::ClipboardErr(format!("{value}"))
    }
}

#[derive(Debug, Default)]
struct Config {
    input_file: Option<String>,
    from_clip: bool,
    toc: bool,
    title_order: bool,
    insert: bool,
    output_file: Option<String>,
    to_clip: bool,
}

fn main() -> Result<(), ErrKind> {
    let mut args = std::env::args().skip(1);
    if args.len() <= 0 {
        println!("{USAGE}");
        return Ok(());
    }
    let mut config: Config = Default::default();
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            "-v" | "--version" => {
                println!("{VERSION}");
                return Ok(());
            }
            "-f" | "--file" => {
                if let Some(file) = args.next() {
                    config.input_file = Some(file);
                } else {
                    return Err(ErrKind::InvalidParam(
                        "`-f, --file` requires a file param".to_owned(),
                    ));
                }
            }
            "-fc" | "--from-clip" => config.from_clip = true,
            "-t" | "--toc" => config.toc = true,
            "-o" | "--title-order" => config.title_order = true,
            "-i" | "--insert" => config.insert = true,
            "-w" | "--write" => {
                if let Some(file) = args.next() {
                    config.output_file = Some(file);
                } else {
                    return Err(ErrKind::InvalidParam(
                        "`-w, --write` requires a file param".to_owned(),
                    ));
                }
            }
            "-tc" | "--to-clip" => config.to_clip = true,
            _ => {}
        }
    }

    verify(&config)?;
    handle(config)
}

fn verify(config: &Config) -> Result<(), ErrKind> {
    if config.from_clip && config.input_file.is_some() {
        return Err(ErrKind::InvalidParam(
            "`-f, --file` conflicts with `-fc, --from-clip`".to_owned(),
        ));
    }
    if !config.to_clip && config.output_file.is_none() {
        return Err(ErrKind::InvalidParam(
            "`-w, --write` or `-tc, --to-clip` is required".to_owned(),
        ));
    }
    Ok(())
}

fn handle(config: Config) -> Result<(), ErrKind> {
    let content = read_markdown(config)?;
    for line in content {
        println!("{line}");
    }
    // println!("{content}");
    Ok(())
}

fn read_markdown(config: Config) -> Result<Box<dyn Iterator<Item = String>>, ErrKind> {
    if config.from_clip {
        clipboard_win::get_clipboard_string()
            .map_err(From::from)
            .map(Cursor::new)
            .map(Box::new)
    } else if let Some(file) = config.input_file {
        File::open(file)
            .map_err(From::from)
            .map(BufReader::new)
            .map(Box::new)
    } else {
        Err(ErrKind::InvalidParam(
            "`-f, --file` or `-fc, --from-clip` is required".to_owned(),
        ))
    }
}
