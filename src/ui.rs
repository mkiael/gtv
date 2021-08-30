use std::io::Write;
use termion::{clear, color, style};

#[derive(PartialEq)]
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
}

pub struct Config {
    pub enable_ui: bool,
    pub only_failed: bool,
}

pub struct Ui {
    config: Config,
    iteration: TestIteration,
}

impl Ui {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            iteration: TestIteration::new(),
        }
    }

    pub fn init_iteration(&mut self, num_cases: i64, num_suites: i64) {
        self.iteration.num_cases = num_cases;
        self.iteration.num_suites = num_suites;
    }

    pub fn add_suite(&mut self, suite: TestSuite) {
        self.iteration.suites.push(suite)
    }

    pub fn add_case(&mut self, case: TestCase) {
        self.iteration.suites.last_mut().unwrap().cases.push(case);
    }

    pub fn update_last_case(&mut self, state: TestState, duration: i64) {
        let mut last_case = self
            .iteration
            .suites
            .last_mut()
            .unwrap()
            .cases
            .last_mut()
            .unwrap();
        last_case.state = state;
        last_case.duration = duration;
    }

    pub fn render(&self) {
        if self.config.enable_ui {
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
                self.iteration.num_cases,
                style::Reset,
                style::Bold,
                self.iteration.num_suites,
                style::Reset,
            )
            .unwrap();
            for suite in self.iteration.suites.iter() {
                let to_render = suite
                    .cases
                    .iter()
                    .filter(|test_case| {
                        if self.config.only_failed {
                            test_case.state == TestState::Failed
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<&TestCase>>();

                if !to_render.is_empty() {
                    writeln!(
                        tty,
                        "Running {}{}{}.",
                        style::Bold,
                        suite.name,
                        style::Reset,
                    )
                    .unwrap();
                }
                for case in to_render.iter() {
                    writeln!(tty, "\t{}{}{}.", style::Bold, case.name, style::Reset,).unwrap();
                }
            }
            tty.flush().unwrap();
        }
    }
}
