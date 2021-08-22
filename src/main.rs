use std::io;
use regex::Regex;

const TEST_ITERATION: &str = "[==========]";
const TEST_SETUP: &str ="[----------]";
const TEST_PASSED: &str = "[  PASSED  ]";
const MARK_SIZE: usize = TEST_ITERATION.len();

enum AppState {
    Inactive,
    TestIterationStarted,
    TestIterationDone,
    TestIterationEnd,
    Failure,
}

fn match_mark(line: &str, mark: &str) -> bool {
    line.len() > MARK_SIZE && &line[..MARK_SIZE] == mark
}

fn parse_test_count(line: &str) -> i64 {
    let re = Regex::new(r"(?P<num_cases>[0-9]+) tests\.").unwrap();
    if let Some(caps) =  re.captures(line) {
        caps["num_cases"].parse::<i64>().unwrap()
    } else {
        0
    }
}

fn process(state: &AppState, line: &str) -> AppState {
    match state {
        AppState::Inactive => {
            if match_mark(line, TEST_ITERATION) {
                eprintln!("Test iteration started");
                AppState::TestIterationStarted
            } else {
                AppState::Inactive
            }
        }
        AppState::TestIterationStarted => {
            if match_mark(line, TEST_ITERATION) {
                eprintln!("Test iteration done");
                AppState::TestIterationDone
            } else {
                AppState::TestIterationStarted
            }
        }
        AppState::TestIterationDone => {
            if match_mark(line, TEST_PASSED) {
                eprintln!("Number of test cases: {}", parse_test_count(line));
                AppState::TestIterationEnd
            }
            else {
                AppState::TestIterationDone
            }
        }
        _ => AppState::Inactive
    }
}

fn main() {
    let mut state = AppState::Inactive;
    loop {
        let mut buffer = String::new();
        state =  match io::stdin().read_line(&mut buffer) {
            Err(_) => AppState::Failure,
            Ok(0) => AppState::Failure,
            Ok(_) => process(&state, &buffer),
        };
        match state {
            AppState::Failure => break,
            _ => (),
        }
    }
    eprintln!("Program done");
}
