pub struct App{
    pub input: Vec<char>,
    pub input_idx: usize,
    pub input_cursor_position: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: vec![],
            input_idx: 0,
            input_cursor_position: 0,

        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Analysis,
    Error,
    HelpMenu,
    Home,
    Input,
    Portfolio,
    BasicView,
}

pub const MAJOR_INDICES: [&str; 3] = [
  "DJIA",
  "Nasdaq",
  "S&P",
];

pub const BANNER: &str = "
   _____ ________
  / ___/    ||
 (__  )     ||
/____/      ||

";

