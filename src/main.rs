use gtv::parser::{parse, ParserEvent};
use std::io::{stdin, BufRead, BufReader, Write};
use std::sync::mpsc;
use std::thread;
use termion::{clear, color, style};

enum TestState {
    Running,
    Passed,
    Failed,
}

struct TestCase {
    name: String,
    state: TestState,
    duration: i64,
}

impl TestCase {
    fn new(name: String) -> Self {
        Self {
            name,
            state: TestState::Running,
            duration: 0,
        }
    }
}

struct TestSuite {
    name: String,
    cases: Vec<TestCase>,
}

impl TestSuite {
    fn new(name: String) -> Self {
        Self {
            name,
            cases: Vec::new(),
        }
    }

    fn add_case(&mut self, case: TestCase) {
        self.cases.push(case)
    }
}

struct TestIteration {
    num_suites: i64,
    num_cases: i64,
    suites: Vec<TestSuite>,
}

impl TestIteration {
    fn new() -> Self {
        Self {
            num_suites: 0,
            num_cases: 0,
            suites: Vec::new(),
        }
    }

    fn add_suite(&mut self, suite: TestSuite) {
        self.suites.push(suite)
    }

    fn last_suite(&mut self) -> &mut TestSuite {
        self.suites.last_mut().unwrap()
    }
}

fn main() {
    let (listener_tx, listener_rx) = mpsc::channel();
    let parser_thd = thread::spawn(move || {
        let mut input_reader: Box<dyn BufRead> = Box::new(BufReader::new(stdin()));
        parse(&mut input_reader, listener_tx).unwrap();
    });

    let mut iteration = TestIteration::new();

    loop {
        match listener_rx.recv().unwrap() {
            ParserEvent::NewIteration(num_cases, num_suites) => {
                iteration.num_cases = num_cases;
                iteration.num_suites = num_suites;
            }
            ParserEvent::NewSuite(num_cases, suite_name) => {
                iteration.add_suite(TestSuite::new(suite_name))
            }
            ParserEvent::NewTestCase(name) => iteration.last_suite().add_case(TestCase::new(name)),
            ParserEvent::PassedTests(num_passed) => {}
            ParserEvent::Done => break,
        }

        let mut tty = termion::get_tty().unwrap();
        writeln!(
            tty,
            "{}{}{}Google Test Viewer{}",
            clear::All,
            style::Bold,
            color::Fg(color::Green),
            style::Reset
        )
        .unwrap();
        writeln!(
            tty,
            "Running {}{}{} from {}{}{} suites.",
            style::Bold,
            iteration.num_cases,
            style::Reset,
            style::Bold,
            iteration.num_suites,
            style::Reset,
        )
        .unwrap();
        for suite in iteration.suites.iter() {
            writeln!(
                tty,
                "Running {}{}{}.",
                style::Bold,
                suite.name,
                style::Reset,
            )
            .unwrap();
            for case in suite.cases.iter() {
                writeln!(tty, "\t{}{}{}.", style::Bold, case.name, style::Reset,).unwrap();
            }
        }
        tty.flush().unwrap();
    }
    parser_thd.join().unwrap();
    eprintln!("Program done");
}
