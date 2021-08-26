use gtv::parser::{parse, ParserEvent};
use std::io::{stdin, BufRead, BufReader};
use std::sync::mpsc;
use std::thread;

fn main() {
    let (listener_tx, listener_rx) = mpsc::channel();
    let parser_thd = thread::spawn(move || {
        let mut input_reader: Box<dyn BufRead> = Box::new(BufReader::new(stdin()));
        parse(&mut input_reader, listener_tx).unwrap();
    });

    loop {
        match listener_rx.recv().unwrap() {
            ParserEvent::NewIteration(num_cases, num_suites) => {
                eprintln!("New iteration {} {}", num_cases, num_suites)
            }
            ParserEvent::NewSuite(num_cases, suite_name) => {
                eprintln!("New suite {} {}", num_cases, suite_name)
            }
            ParserEvent::Done => break,
        }
    }
    parser_thd.join().unwrap();
    eprintln!("Program done");
}
