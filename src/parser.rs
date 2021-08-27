use regex::Regex;
use std::io::{BufRead, Error, ErrorKind, Result};
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub enum ParserEvent {
    NewIteration(i64, i64),
    NewSuite(i64, String),
    NewTestCase(String),
    PassedTests(i64),
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
    TestCaseStart,
    TestCaseEnd,
}

const TEST_ITERATION: &str = "[==========]";
const TEST_SETUP: &str = "[----------]";
const TEST_SUITE: &str = "[----------]";
const TEST_RUN: &str = "[ RUN      ]";
const TEST_OK: &str = "[       OK ]";
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
            ParserState::IterationEnd => {
                if match_mark(line, TEST_PASSED) {
                    if let Some(num_passed) = parse_num_tests(strip_mark(line)) {
                        self.listener
                            .send(ParserEvent::PassedTests(num_passed))
                            .unwrap();
                    }
                }
                Ok(())
            }
            ParserState::SetupStart | ParserState::SuiteEnd => {
                if match_mark(line, TEST_SUITE) {
                    if let Some(((num_tests, suite_name))) = parse_test_suite(strip_mark(line)) {
                        self.state = ParserState::SuiteStart;
                        self.listener
                            .send(ParserEvent::NewSuite(num_tests, suite_name))
                            .unwrap();
                    } else {
                        // TODO: Check for environment tear-down
                        self.state = ParserState::SetupEnd;
                    }
                }
                Ok(())
            }
            ParserState::SetupEnd => {
                if match_mark(line, TEST_ITERATION) {
                    self.state = ParserState::IterationEnd;
                }
                Ok(())
            }
            ParserState::SuiteStart | ParserState::TestCaseEnd => {
                if match_mark(line, TEST_RUN) {
                    self.state = ParserState::TestCaseStart;
                    self.listener
                        .send(ParserEvent::NewTestCase(
                            strip_mark(line).trim().to_string(),
                        ))
                        .unwrap();
                } else if match_mark(line, TEST_SUITE) {
                    self.state = ParserState::SuiteEnd;
                }
                Ok(())
            }
            ParserState::TestCaseStart => {
                if match_mark(line, TEST_OK) {
                    self.state = ParserState::TestCaseEnd;
                }
                Ok(())
            }
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
    &line[MARK_SIZE..].trim()
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

fn parse_test_suite(line: &str) -> Option<(i64, String)> {
    let re = Regex::new(r"(?P<num_cases>[0-9]+) tests from (?P<suite_name>[a-zA-Z_$][a-zA-Z\d_]+)")
        .unwrap();
    if let Some(caps) = re.captures(line) {
        Some((
            caps["num_cases"].parse::<i64>().unwrap(),
            caps["suite_name"].to_string(),
        ))
    } else {
        None
    }
}

fn parse_num_tests(line: &str) -> Option<i64> {
    let re = Regex::new(r"(?P<num_tests>[0-9]+) tests.").unwrap();
    if let Some(caps) = re.captures(line) {
        Some(caps["num_tests"].parse::<i64>().unwrap())
    } else {
        None
    }
}
