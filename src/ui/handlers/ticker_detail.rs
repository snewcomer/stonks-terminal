use super::{
    super::super::app::{ActiveBlock, App, RouteId},
    super::key::Key,
    common_key_events,
};
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        Key::Esc => {
            if let (Some(_tickers), Some(_selected_ticker_index)) =
                (&app.search_results.tickers, &app.search_results.selected_ticker_index)
                {
                    // On searching for a track, clear the ticker selection
                    app.search_results.selected_ticker_index = Some(0);

                    // Default fallback behavior: treat the input as a raw search phrase.
                    app.push_navigation_stack(RouteId::Search, ActiveBlock::SearchResults);
                };
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
