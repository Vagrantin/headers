use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self,BufRead,BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Matthieu Ducorps <mducorps@gamil.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
            .short("n")
            .long("lines")
            .value_name("LINES")
            .help("number of lines to print")
            .takes_value(true)
            .default_value("10"),
            )
        .arg(
            Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .value_name("BYTES")
            .help("ouput characters base on number of bytes provided")
            .takes_value(true)
            .conflicts_with("lines"),
            )
        .get_matches();
    let lines = matches.value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches.value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;
    Ok(Config {
        files:matches.values_of_lossy("files").unwrap(),
        lines:lines.unwrap(),
        bytes
    })
}
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val))
    }

}

#[test]
fn test_parse_positive_int() {
    //3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());

    //Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
}
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}:{}", filename, err),
//            Ok(_) => println!("Opened {}", filename),
            Ok(file) => {
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    let mut counter = 1;
                    let maxLine = file.lines().enumerate();
                        println!("{}", maxLine);
                    while counter <= config.lines {
                        println!("{}", line);
                        counter += 1;
                    }
//                    if config.lines == 10 {
//                        println!("{}",line);
//                    } else {
//                        println!("c'est pas de la lignes");
//                    }
                }
            }
        }
    }
    //println!("{:#?}", config);
    Ok(())
}
