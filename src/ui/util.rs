use crate::config::Theme;
use tui::style::Style;

pub fn get_color((is_active, is_hovered): (bool, bool), theme: Theme) -> Style {
  match (is_active, is_hovered) {
    (true, _) => Style::default().fg(theme.selected),
    (false, true) => Style::default().fg(theme.hovered),
    _ => Style::default().fg(theme.inactive),
  }
}

// `percentage` param needs to be between 0 and 1
pub fn get_percentage_width(width: u16, percentage: f32) -> u16 {
  let padding = 3;
  let width = width - padding;
  (f32::from(width) * percentage) as u16
}

