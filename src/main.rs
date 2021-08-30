use gtv::parser::{parse, ParserEvent};
use gtv::ui::{Config, TestCase, TestState, TestSuite, Ui};
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
    let mut ui = Ui::new(Config {
        enable_ui: env::var("GTV_NO_UI").unwrap_or_default().is_empty(),
        only_failed: true,
    });

    loop {
        match listener_rx.recv().unwrap() {
            ParserEvent::NewIteration(num_cases, num_suites) => {
                ui.init_iteration(num_cases, num_suites);
            }
            ParserEvent::NewSuite(_num_cases, suite_name) => {
                ui.add_suite(TestSuite::new(suite_name));
            }
            ParserEvent::NewTestCase(name) => ui.add_case(TestCase::new(name)),
            ParserEvent::TestCasePassed(time) => {
                ui.update_last_case(TestState::Passed, time);
            }
            ParserEvent::TestCaseFailed(_reason, time) => {
                ui.update_last_case(TestState::Failed, time);
            }
            ParserEvent::PassedTests(_num_passed) => {}
            ParserEvent::Done => break,
        }
    }

    ui.render();

    parser_thd.join().unwrap();
    eprintln!("Program done");
}
