use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::event::{poll, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute, QueueableCommand};

pub struct Editor {
    should_run: bool,
}

impl Editor {
    pub fn defaut() -> Editor {
        Self { should_run: true }
    }

    fn _die(e: &std::io::Error) {
        panic!("{e:?}");
    }

    pub fn run(mut self) {
        let mut stdout = stdout();

        enable_raw_mode().unwrap();

        let (mut width, mut height) = crossterm::terminal::size().unwrap();


        let mut content = String::new();

        //Set up the editor [Clearing and moving cursor]
        execute!(stdout, Clear(ClearType::All)).unwrap();
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        while self.should_run {
            if poll(Duration::from_millis(1)).unwrap() {
                match crossterm::event::read().unwrap() {
                    Event::Key(event) => match event.code {
                        KeyCode::Char(character) => {
                            if event.modifiers == KeyModifiers::CONTROL && character == 'q' {
                                self.should_run = false;
                            } else {
                                content.push(character);
                            }
                        }
                        KeyCode::Enter => {
                            content.push('\r');
                            content.push('\n');
                        }
                        KeyCode::Backspace => {
                            content.pop();
                        }
                        _ => {}
                    },
                    Event::Resize(nwidth, nheight) => {
                        println!("New size {}x{}", width, height);
                        width = nwidth;
                        height = nheight;
                    }
                    _ => println!("ERROR"),
                }
            }
            stdout.queue(Clear(ClearType::All)).unwrap();
            execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

            stdout.write(content.as_bytes()).unwrap();
            stdout.flush().unwrap();
        }
        disable_raw_mode().unwrap();
    }
}
