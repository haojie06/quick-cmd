use std::{
    io::{self, stdout},
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use anyhow::{Ok, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub enum Event {
    Tick,
    Key(KeyEvent),
}

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
    #[allow(dead_code)]
    join_handle: JoinHandle<()>,
    #[allow(dead_code)]
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
}

impl Tui {
    pub fn new(tick_rate: u16) -> Self {
        let terminal =
            Terminal::new(CrosstermBackend::new(stdout())).expect("failed to create terminal");
        let (event_sender, event_receiver) = mpsc::channel();
        let tick_interval = Duration::from_secs_f32(1.0 / tick_rate as f32);
        let mut last_tick = Instant::now();
        let join_handle = {
            let event_sender = event_sender.clone();
            thread::spawn(move || loop {
                let poll_timeout = tick_interval
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_interval);
                if event::poll(poll_timeout).expect("failed to poll events") {
                    match event::read().expect("failed to read event") {
                        event::Event::Key(k_event) => {
                            // 避免某些系统按键触发两次
                            if k_event.kind == event::KeyEventKind::Press {
                                event_sender
                                    .send(Event::Key(k_event))
                                    .expect("failed to send event");
                            }
                        }
                        _ => (),
                    }
                }
                if last_tick.elapsed() >= tick_interval {
                    event_sender
                        .send(Event::Tick)
                        .expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            })
        };
        Self {
            terminal,
            join_handle,
            event_sender,
            event_receiver,
        }
    }

    pub fn enter() -> Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn exit() -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        // cursor?
        Ok(())
    }

    pub fn next_event(&self) -> Result<Event> {
        Ok(self.event_receiver.recv()?)
    }
}
