use std::io::{stdout, Stdout, Write};
use std::time::Duration;

use crossterm::event::{poll, Event, KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute, QueueableCommand};

pub struct Editor {
    should_run: bool,

    width: u16,
    height: u16,
}

impl Editor {
    pub fn default() -> Editor {
        let (width, height) = crossterm::terminal::size().unwrap();

        Self {
            should_run: true,
            width: width,
            height: height,
        }
    }

    fn _die(e: &std::io::Error) {
        panic!("{e:?}");
    }


    fn welcome_window(&self, mut stdout: &Stdout) {


        for i in 0..self.height {
            stdout.queue(cursor::MoveTo(0, i)).unwrap();
            stdout.write("~".as_bytes()).unwrap();
        }

        let message = vec![
            "Welcome to TextX",
            "For more info type :help",
            "Command is not supported yet",
        ];

        let message_count: u16 = message.len() as u16;

        for (index, item) in message.iter().enumerate() {
            stdout
                .queue(cursor::MoveTo(
                    self.width.div_ceil(2) - (item.len() as u16).div_ceil(2),
                    self.height.div_ceil(2) - message_count + index as u16,
                ))
                .unwrap();

            stdout.queue(Print(item)).unwrap();
        }
        stdout.queue(cursor::Hide).unwrap();
        stdout.flush().unwrap();

    }

    pub fn run(mut self) {
        let mut stdout = stdout();

        enable_raw_mode().unwrap();
       
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
                        println!("New size {}x{}", self.width, self.height);

                        self.width = nwidth;
                        self.height = nheight;

                        stdout.queue(Clear(ClearType::All)).unwrap();
                        stdout.queue(Clear(ClearType::Purge)).unwrap();
                    }
                    _ => println!("ERROR"),
                }
            }
            self.welcome_window(&stdout);
        }
    }
}



// Clean Up Section
impl Drop for Editor {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().queue(cursor::Show).unwrap();
        stdout().queue(Clear(ClearType::All)).unwrap();
        stdout().queue(Clear(ClearType::Purge)).unwrap();

        stdout().queue(cursor::MoveTo(0, 0)).unwrap();
    }
}
