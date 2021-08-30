use gtv::parser::{parse, ParserEvent};
use gtv::ui::{Config, TestCase, TestIteration, TestState, TestSuite, Ui};
use std::env;
use std::io::{stdin, BufRead, BufReader};
use std::sync::mpsc;
use std::thread;

fn main() {
    let (listener_tx, listener_rx) = mpsc::channel();
    let parser_thd = thread::spawn(move || {
        let mut input_reader: Box<dyn BufRead> = Box::new(BufReader::new(stdin()));
        parse(&mut input_reader, listener_tx).unwrap();
    });
    let mut iteration = TestIteration::new();
    let ui = Ui::new(Config {
        enable_ui: env::var("GTV_NO_UI").unwrap_or_default().is_empty(),
        only_failed: true,
    });

    loop {
        match listener_rx.recv().unwrap() {
            ParserEvent::NewIteration(num_cases, num_suites) => {
                iteration.num_cases = num_cases;
                iteration.num_suites = num_suites;
            }
            ParserEvent::NewSuite(_num_cases, suite_name) => {
                iteration.add_suite(TestSuite::new(suite_name));
            }
            ParserEvent::NewTestCase(name) => iteration.last_suite().add_case(TestCase::new(name)),
            ParserEvent::TestCasePassed(time) => {
                let mut test_case = iteration.last_suite().last_case();
                test_case.state = TestState::Passed;
                test_case.duration = time;
            }
            ParserEvent::TestCaseFailed(_reason, time) => {
                let mut test_case = iteration.last_suite().last_case();
                test_case.state = TestState::Failed;
                test_case.duration = time;
            }
            ParserEvent::PassedTests(_num_passed) => {}
            ParserEvent::Done => break,
        }
    }

    ui.render(&iteration);

    parser_thd.join().unwrap();
    eprintln!("Program done");
}
