use gtv::parser::{parse, ParserEvent};
use std::io::{stdin, BufRead, BufReader, Write};
use std::sync::mpsc;
use std::thread;
use termion::{clear, color, style};

fn main() {
    let (listener_tx, listener_rx) = mpsc::channel();
    let parser_thd = thread::spawn(move || {
        let mut input_reader: Box<dyn BufRead> = Box::new(BufReader::new(stdin()));
        parse(&mut input_reader, listener_tx).unwrap();
    });

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
    tty.flush().unwrap();

    loop {
        match listener_rx.recv().unwrap() {
            ParserEvent::NewIteration(num_cases, num_suites) => {
                writeln!(
                    tty,
                    "Running a total of {} test cases in {} test suites.",
                    num_cases, num_suites
                )
                .unwrap();
                tty.flush().unwrap();
            }
            ParserEvent::NewSuite(num_cases, suite_name) => {
                writeln!(
                    tty,
                    "Running {}{}{} with {} tests.",
                    style::Bold,
                    suite_name,
                    style::Reset,
                    num_cases
                )
                .unwrap();
                tty.flush().unwrap();
            }
            ParserEvent::NewTestCase(name) => {
                writeln!(tty, "Running test {}{}{}.", style::Bold, name, style::Reset,).unwrap();
                tty.flush().unwrap();
            }
            ParserEvent::Done => break,
        }
    }
    parser_thd.join().unwrap();
    eprintln!("Program done");
}
