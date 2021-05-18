use super::{
    super::super::app::{App},
    super::key::Key,
    common_key_events,
};
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        k if common_key_events::down_event(k) => {
            match &app.search_results.tickers {
                Some(p) => {
                    if let Some(selected_ticker_index) = app.search_results.selected_ticker_index {
                        let next_index =
                            common_key_events::on_down_press_handler(&p, Some(selected_ticker_index));
                        app.search_results.selected_ticker_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        k if common_key_events::up_event(k) => {
            match &app.search_results.tickers {
                Some(p) => {
                    let next_index =
                        common_key_events::on_up_press_handler(&p, app.search_results.selected_ticker_index);
                    app.search_results.selected_ticker_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::high_event(k) => {
            match &app.search_results.tickers {
                Some(_p) => {
                    let next_index = common_key_events::on_high_press_handler();
                    app.search_results.selected_ticker_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::middle_event(k) => {
            match &app.search_results.tickers {
                Some(p) => {
                    let next_index = common_key_events::on_middle_press_handler(&p);
                    app.search_results.selected_ticker_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::low_event(k) => {
            match &app.search_results.tickers {
                Some(p) => {
                    let next_index = common_key_events::on_low_press_handler(&p);
                    app.search_results.selected_ticker_index = Some(next_index);
                }
                None => {}
            };
        }
        Key::Enter => {
            if let (Some(tickers), Some(selected_ticker_index)) =
                (&app.search_results.tickers, &app.search_results.selected_ticker_index)
                {
                    app.active_ticker_index = Some(selected_ticker_index.to_owned());
                    if let Some(selected_ticker) = tickers.get(selected_ticker_index.to_owned()) {
                        let ticker_id = selected_ticker.symbol.to_owned();
                        app.dispatch(IoEvent::GetTicker(ticker_id));
                    }
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
