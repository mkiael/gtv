use clap::{App, Arg};
use gtv::parser::{parse, ParserEvent};
use gtv::ui::{Config, TestCase, TestState, TestSuite, Ui};
use std::io::{stdin, BufRead, BufReader};
use std::sync::mpsc;
use std::thread;

fn main() {
    let args = App::new("Google Test Viewer")
        .version("0.1.0")
        .arg(
            Arg::with_name("only-failed")
                .short("f")
                .long("failed")
                .help("Only print failed tests")
        )
        .get_matches();

    let config = Config {
        only_failed: args.is_present("only-failed"),
    };

    let (listener_tx, listener_rx) = mpsc::channel();
    let parser_thd = thread::spawn(move || {
        let mut input_reader: Box<dyn BufRead> = Box::new(BufReader::new(stdin()));
        parse(&mut input_reader, listener_tx).unwrap();
    });

    let mut ui = Ui::new(config);
    loop {
        match listener_rx.recv().unwrap() {
            ParserEvent::NewIteration(num_cases, num_suites) => {
                ui.init_iteration(num_cases, num_suites);
            }
            ParserEvent::NewSuite(_num_cases, suite_name) => {
                ui.add_suite(TestSuite::new(suite_name));
            }
            ParserEvent::TestCasePassed(test_name, duration) => {
                ui.add_case(TestCase::new(test_name, duration, TestState::Passed));
            }
            ParserEvent::TestCaseFailed(test_name, duration, reason) => {
                ui.add_case(TestCase::new(
                    test_name,
                    duration,
                    TestState::Failed(reason),
                ));
            }
            ParserEvent::PassedTests(_num_passed) => {}
            ParserEvent::Done => break,
        }
    }

    ui.render();

    parser_thd.join().unwrap();
}
