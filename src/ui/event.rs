use super::key::Key;
use crossterm::event;
use std::{sync::mpsc, thread, time::Duration};

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _sx: mpsc::Sender<Event<Key>>,
}

impl Events {
    pub fn new() -> Self {
        let (sx, rx) = mpsc::channel();

        let event_sx = sx.clone();
        thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(75)).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);

                        event_sx.send(Event::Input(key)).unwrap();
                    }
                }

                event_sx.send(Event::Tick).unwrap();
            }
        });

        Self { rx, _sx: sx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        // https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv
        self.rx.recv()
    }
}
