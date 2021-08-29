use gtv::parser::{parse, ParserEvent};
use gtv::ui::{render, TestCase, TestIteration, TestSuite};
use std::env;
use std::io::{stdin, BufRead, BufReader};
use std::sync::mpsc;
use std::thread;

fn main() {
    let enable_ui = env::var("GTV_NO_UI").unwrap_or_default().is_empty();

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

        if enable_ui {
            render(&iteration);
        }
    }
    parser_thd.join().unwrap();
    eprintln!("Program done");
}
