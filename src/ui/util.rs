use crate::config::Theme;
use chrono::prelude::*;
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

pub fn date_from_timestamp(seconds: i64) -> String {
        // Convert the timestamp string into an i64
    let timestamp = seconds;

    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d");

    newdate.to_string()
}

