use regex::Regex;
use std::io::{BufRead, Error, ErrorKind, Result};
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub enum ParserEvent {
    NewIteration(i64, i64),
    NewSuite(i64, String),
    Done,
}

enum ParserState {
    Inactive,
    IterationStart,
    IterationEnd,
    SetupStart,
    SetupEnd,
    SuiteStart,
    SuiteEnd,
}

const TEST_ITERATION: &str = "[==========]";
const TEST_SETUP: &str = "[----------]";
const TEST_SUITE: &str = "[----------]";
const TEST_PASSED: &str = "[  PASSED  ]";
const MARK_SIZE: usize = TEST_ITERATION.len();

struct Parser {
    state: ParserState,
    listener: Sender<ParserEvent>,
}

impl Parser {
    fn new(listener: Sender<ParserEvent>) -> Self {
        Self {
            state: ParserState::Inactive,
            listener,
        }
    }

    fn process_line(&mut self, line: &str) -> Result<()> {
        match self.state {
            ParserState::Inactive => {
                if match_mark(line, TEST_ITERATION) {
                    match parse_test_count(strip_mark(line)) {
                        Ok((num_cases, num_suites)) => {
                            self.state = ParserState::IterationStart;
                            self.listener
                                .send(ParserEvent::NewIteration(num_cases, num_suites))
                                .unwrap();
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            ParserState::IterationStart => {
                if match_mark(line, TEST_SETUP) {
                    self.state = ParserState::SetupStart;
                }
                Ok(())
            }
            ParserState::IterationEnd => Ok(()),
            ParserState::SetupStart => {
                if match_mark(line, TEST_SUITE) {
                    match parse_test_suite(strip_mark(line)) {
                        Ok((num_tests, suite_name)) => {
                            self.state = ParserState::SuiteStart;
                            self.listener
                                .send(ParserEvent::NewSuite(num_tests, suite_name))
                                .unwrap();
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            ParserState::SetupEnd => Ok(()),
            ParserState::SuiteStart => Ok(()),
            ParserState::SuiteEnd => Ok(()),
        }
    }

    fn finalize(&mut self) {
        self.listener.send(ParserEvent::Done).unwrap();
    }
}

pub fn parse(input: &mut Box<dyn BufRead>, listener: Sender<ParserEvent>) -> Result<()> {
    let mut parser = Parser::new(listener);
    loop {
        let mut line = String::new();
        match input.read_line(&mut line) {
            Err(_) => break,
            Ok(0) => break,
            Ok(_) => match parser.process_line(&line) {
                Err(_) => break,
                _ => (),
            },
        }
    }
    // TODO: Return proper value
    parser.finalize();
    Ok(())
}

fn match_mark(line: &str, mark: &str) -> bool {
    line.len() > MARK_SIZE && &line[..MARK_SIZE] == mark
}

fn strip_mark(line: &str) -> &str {
    &line[(MARK_SIZE+1)..]
}

fn parse_test_count(line: &str) -> Result<(i64, i64)> {
    let re = Regex::new(
        r"Running (?P<num_cases>[0-9]+) tests from (?P<num_suites>[0-9]+) test suites\.",
    )
    .unwrap();
    if let Some(caps) = re.captures(line) {
        Ok((
            caps["num_cases"].parse::<i64>().unwrap(),
            caps["num_suites"].parse::<i64>().unwrap(),
        ))
    } else {
        Err(Error::new(ErrorKind::Other, "Parsing test count failed."))
    }
}

fn parse_test_suite(line: &str) -> Result<(i64, String)> {
    let re = Regex::new(r"(?P<num_cases>[0-9]+) tests from (?P<suite_name>[a-zA-Z_$][a-zA-Z\d_]+)")
        .unwrap();
    if let Some(caps) = re.captures(line) {
        Ok((
            caps["num_cases"].parse::<i64>().unwrap(),
            caps["suite_name"].to_string(),
        ))
    } else {
        Err(Error::new(ErrorKind::Other, "Parsing test suite failed."))
    }
}
