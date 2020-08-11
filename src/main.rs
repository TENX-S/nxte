extern crate termion;


use std::io::{ Write, stdout, stdin };
use termion::{ event::Key, screen::*, input::TermRead, raw::IntoRawMode };


fn main() {

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut screen = AlternateScreen::from(stdout);
    writeln!(screen, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::Hide).unwrap();

    for c in stdin.keys() {

        write!(screen, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c)   => println!("{}", c),
            Key::Alt(c)    => println!("Alt-{}", c),
            Key::Ctrl(c)   => println!("Ctrl-{}", c),
            Key::Left      => println!("<left>"),
            Key::Right     => println!("<right>"),
            Key::Up        => println!("<up>"),
            Key::Down      => println!("<down>"),
            _              => println!("Other"),
        }

        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}