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
        .author("Matthieu Ducorps <mail@gamil.com>")
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
        .map_err(|e| format!("invalid value '{}' for '--lines <LINES>': invalid digit found in string", e))?;

    let bytes = matches.value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("invalid value '{}' for '--bytes <BYTES>': invalid digit found in string", e))?;
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match config.bytes {
// we go throuh this branch if this is character output requested ( -c )
            Some(_int) => { 
                 match open(&filename) {
                     Err(_err) => eprintln!("{}: .* [(]os error 2[)]", filename),
                     Ok(mut file) => {
                        let max_char = config.bytes;
                        let index = &config.files.iter().position(|r| r == filename).unwrap();
                        let mut i = 0;
                        let mut mychar = vec![];
                        let number_of_files = &config.files.len();
                        let first_file: usize = 0;
                        if *number_of_files > 1 {
                            if index == &first_file {
                               println!("==> {} <==", &filename);
                            } else {
                               println!("\n==> {} <==", &filename);
                            }
                        }
                        let mut line = String::new();
                        loop {
                            line.clear();
                            if 0 == file.read_line(&mut line)? || i == max_char.unwrap()   {
                                break;
                            }
                            for byte in line.bytes(){
                                mychar.push(byte);
                                i += 1;
                                if byte == b'\n' || byte == b'\r' || byte == 116 || i == max_char.unwrap(){
                                    break;
                                }
                            }
                        }
                        print!("{}",String::from_utf8_lossy(&mychar));
                     }
                     }
                 },
//Alternatively we go through this branch which is line output.
            None => {
                 match open(&filename) {
                     Err(_err) => eprintln!("{}: .* [(]os error 2[)]", filename),
                     Ok(mut file) => {
                         let max_output_line = &config.lines;
                         let index = &config.files.iter().position(|r| r == filename).unwrap();
                         let number_of_files = &config.files.len();
                         let first_file: usize = 0;
                         if *number_of_files > 1 {
                            if index == &first_file {
                               println!("==> {} <==", &filename);
                            } else {
                               println!("\n==> {} <==", &filename);
                            }
                         }
                        let mut line_num = 0;
                        let mut line = String::new();
                        loop {
                            line.clear();
                            if 0 == file.read_line(&mut line)? || &line_num == max_output_line {
                                break;
                            }
                            print!("{}", line);
                            line_num += 1;
                        }
                     }
                     }
                 },
         }
    }
//    println!("{:#?}", &config);
    Ok(())
}
