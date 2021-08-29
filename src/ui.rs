use std::io::Write;
use termion::{clear, color, style};

pub enum TestState {
    Running,
    Passed,
    Failed,
}

pub struct TestCase {
    pub name: String,
    pub state: TestState,
    pub duration: i64,
}

impl TestCase {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: TestState::Running,
            duration: 0,
        }
    }
}

pub struct TestSuite {
    pub name: String,
    pub cases: Vec<TestCase>,
}

impl TestSuite {
    pub fn new(name: String) -> Self {
        Self {
            name,
            cases: Vec::new(),
        }
    }

    pub fn add_case(&mut self, case: TestCase) {
        self.cases.push(case)
    }
}

pub struct TestIteration {
    pub num_suites: i64,
    pub num_cases: i64,
    pub suites: Vec<TestSuite>,
}

impl TestIteration {
    pub fn new() -> Self {
        Self {
            num_suites: 0,
            num_cases: 0,
            suites: Vec::new(),
        }
    }

    pub fn add_suite(&mut self, suite: TestSuite) {
        self.suites.push(suite)
    }

    pub fn last_suite(&mut self) -> &mut TestSuite {
        self.suites.last_mut().unwrap()
    }
}

pub fn render(iteration: &TestIteration) {
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
