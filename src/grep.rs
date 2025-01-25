use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use regex::Regex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Grep {}

impl Grep {

    fn highlight(& self, line: &str, regex: &Regex, stdout: &mut StandardStream) {
        let mut last_index = 0;

        for m in regex.find_iter(line) {
            let (start, end) = (m.start(), m.end());
            write!(stdout, "{}", &line[last_index..start]).unwrap();
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();

            write!(stdout, "{}", &line[start..end]).unwrap();
            stdout.reset().unwrap();

            last_index = end;
        }

        writeln!(stdout, "{}", &line[last_index..]).unwrap();
    }

    fn process(& self, regex: Regex, path: String) {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        for line in reader.lines() {
            let line = line.unwrap();
            if regex.is_match(&line) {
                self.highlight(&line, &regex, &mut stdout);
            }
        }
    }

    pub fn run(& self) {
        let stdin = stdin();
        let mut stdout = stdout();
        let mut input = String::new();

        loop {
            print!("CMD$ ");
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).unwrap();

            let args : Vec<&str> = input.trim().split_ascii_whitespace().collect();

            if args.is_empty() {
                continue;
            }

            if args[0] == "exit" {
                break;
            }

            if args.len() < 2 {
                eprint!("Usage: grep <pattern> <file> \n");
                continue;
            }

            let pattern = args[1];
            let regex = match Regex::new(pattern) {
                Ok(re) => re,
                Err(e) => {
                    eprintln!("Invalid pattern: {}", e);
                    continue;
                }
            };

            self.process(regex, args[2].to_string());
        }
    }
}