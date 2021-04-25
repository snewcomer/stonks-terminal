extern crate unicode_width;

use super::super::super::app::{ActiveBlock, App, OrderFormState, RouteId};
use crate::ui::key::Key;
use crate::network::IoEvent;
use std::convert::TryInto;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

// Handle event when the search input block is active
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('k') => {
            app.input.drain(app.input_idx..app.input.len());
        }
        Key::Ctrl('u') => {
            app.input.drain(..app.input_idx);
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('l') => {
            app.input = vec![];
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('w') => {
            if app.input_cursor_position == 0 {
                return;
            }
            let word_end = match app.input[..app.input_idx].iter().rposition(|&x| x != ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let word_start = match app.input[..word_end].iter().rposition(|&x| x == ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let deleted: String = app.input[word_start..app.input_idx].iter().collect();
            let deleted_len: u16 = UnicodeWidthStr::width(deleted.as_str()).try_into().unwrap();
            app.input.drain(word_start..app.input_idx);
            app.input_idx = word_start;
            app.input_cursor_position -= deleted_len;
        }
        Key::End | Key::Ctrl('e') => {
            app.input_idx = app.input.len();
            let input_string: String = app.input.iter().collect();
            app.input_cursor_position = UnicodeWidthStr::width(input_string.as_str())
                .try_into()
                .unwrap();
        }
        Key::Home | Key::Ctrl('a') => {
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Left | Key::Ctrl('b') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input[app.input_idx - 1];
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Right | Key::Ctrl('f') => {
            if app.input_idx < app.input.len() {
                let next_c = app.input[app.input_idx];
                app.input_idx += 1;
                app.input_cursor_position += compute_character_width(next_c);
            }
        }
        Key::Esc => {
            if let OrderFormState::Quantity = app.order_form_state {
                app.order_form_state = OrderFormState::Initial;
                app.cancel_preview_order();

                // dbl pop to get past initial Order Form State as well from 'B' in TickerDetail
                // handler - app.push_navigation_stack(RouteId::OrderForm, ActiveBlock::OrderForm);
                app.pop_navigation_stack();
                app.pop_navigation_stack();
            } else {
                app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::WatchList));
            }
        }
        Key::Enter => {
            let input_str: String = app.input.iter().collect();

            process_input(app, input_str);
        }
        Key::Char(c) => {
            app.input.insert(app.input_idx, c);
            app.input_idx += 1;
            app.input_cursor_position += compute_character_width(c);
        }
        Key::Backspace | Key::Ctrl('h') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input.remove(app.input_idx - 1);
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Delete | Key::Ctrl('d') => {
            if !app.input.is_empty() && app.input_idx < app.input.len() {
                app.input.remove(app.input_idx);
            }
        }
        _ => {}
    }
}

fn process_input(app: &mut App, input: String) {
    // Don't do anything if there is no input
    if input.is_empty() {
        return;
    }

    let current_route = app.get_current_route();
    match current_route.id {
        RouteId::OrderForm => {
            match app.order_form_state {
                OrderFormState::Quantity => {
                    app.add_next_order_field("quantity", input);
                    app.order_form_state = OrderFormState::Submit;
                    app.input = vec![];
                    app.input_idx = 0;
                    app.input_cursor_position = 0;
                    app.push_navigation_stack(RouteId::OrderForm, ActiveBlock::OrderForm);
                }
                _ => {}
            }
        }
        _ => {
            // On searching for a track, clear the ticker selection
            app.search_results.selected_ticker_index = Some(0);

            // Default fallback behavior: treat the input as a raw search phrase.
            app.dispatch(IoEvent::GetSearchResults(input));
            app.push_navigation_stack(RouteId::Search, ActiveBlock::SearchResults);
        }
    }
}

fn compute_character_width(character: char) -> u16 {
    UnicodeWidthChar::width(character)
        .unwrap()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_vec_char(s: &str) -> Vec<char> {
        String::from(s).chars().collect()
    }

    #[test]
    fn test_compute_character_width_with_multiple_characters() {
        assert_eq!(1, compute_character_width('a'));
        assert_eq!(1, compute_character_width('ß'));
        assert_eq!(1, compute_character_width('ç'));
    }

    #[test]
    fn test_input_handler_clear_input_on_ctrl_l() {
        let mut app = App::default();

        app.input = str_to_vec_char("My text");

        handler(Key::Ctrl('l'), &mut app);

        assert_eq!(app.input, str_to_vec_char(""));
    }

    #[test]
    fn test_input_handler_ctrl_u() {
        let mut app = App::default();

        app.input = str_to_vec_char("My text");

        handler(Key::Ctrl('u'), &mut app);
        assert_eq!(app.input, str_to_vec_char("My text"));

        app.input_cursor_position = 3;
        app.input_idx = 3;
        handler(Key::Ctrl('u'), &mut app);
        assert_eq!(app.input, str_to_vec_char("text"));
    }

    #[test]
    fn test_input_handler_ctrl_k() {
        let mut app = App::default();

        app.input = str_to_vec_char("My text");

        handler(Key::Ctrl('k'), &mut app);
        assert_eq!(app.input, str_to_vec_char(""));

        app.input = str_to_vec_char("My text");
        app.input_cursor_position = 2;
        app.input_idx = 2;
        handler(Key::Ctrl('k'), &mut app);
        assert_eq!(app.input, str_to_vec_char("My"));

        handler(Key::Ctrl('k'), &mut app);
        assert_eq!(app.input, str_to_vec_char("My"));
    }

    #[test]
    fn test_input_handler_ctrl_w() {
        let mut app = App::default();

        app.input = str_to_vec_char("My text");

        handler(Key::Ctrl('w'), &mut app);
        assert_eq!(app.input, str_to_vec_char("My text"));

        app.input_cursor_position = 3;
        app.input_idx = 3;
        handler(Key::Ctrl('w'), &mut app);
        assert_eq!(app.input, str_to_vec_char("text"));
        assert_eq!(app.input_cursor_position, 0);
        assert_eq!(app.input_idx, 0);

        app.input = str_to_vec_char("    ");
        app.input_cursor_position = 3;
        app.input_idx = 3;
        handler(Key::Ctrl('w'), &mut app);
        assert_eq!(app.input, str_to_vec_char(" "));
        assert_eq!(app.input_cursor_position, 0);
        assert_eq!(app.input_idx, 0);
        app.input_cursor_position = 1;
        app.input_idx = 1;
        handler(Key::Ctrl('w'), &mut app);
        assert_eq!(app.input, str_to_vec_char(""));
        assert_eq!(app.input_cursor_position, 0);
        assert_eq!(app.input_idx, 0);

        app.input = str_to_vec_char("Hello there  ");
        app.input_cursor_position = 13;
        app.input_idx = 13;
        handler(Key::Ctrl('w'), &mut app);
        assert_eq!(app.input, str_to_vec_char("Hello "));
        assert_eq!(app.input_cursor_position, 6);
        assert_eq!(app.input_idx, 6);
    }

    #[test]
    fn test_input_handler_esc_back_to_playlist() {
        let mut app = App::default();

        app.set_current_route_state(Some(ActiveBlock::MyPlaylists), None);
        handler(Key::Esc, &mut app);

        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Empty);
    }

    #[test]
    fn test_input_handler_on_enter_text() {
        let mut app = App::default();

        app.input = str_to_vec_char("My tex");
        app.input_cursor_position = app.input.len().try_into().unwrap();
        app.input_idx = app.input.len();

        handler(Key::Char('t'), &mut app);

        assert_eq!(app.input, str_to_vec_char("My text"));
    }

    #[test]
    fn test_input_handler_backspace() {
        let mut app = App::default();

        app.input = str_to_vec_char("My text");
        app.input_cursor_position = app.input.len().try_into().unwrap();
        app.input_idx = app.input.len();

        handler(Key::Backspace, &mut app);
        assert_eq!(app.input, str_to_vec_char("My tex"));

        // Test that backspace deletes from the cursor position
        app.input_idx = 2;
        app.input_cursor_position = 2;

        handler(Key::Backspace, &mut app);
        assert_eq!(app.input, str_to_vec_char("M tex"));

        app.input_idx = 1;
        app.input_cursor_position = 1;

        handler(Key::Ctrl('h'), &mut app);
        assert_eq!(app.input, str_to_vec_char(" tex"));
    }

    #[test]
    fn test_input_handler_delete() {
        let mut app = App::default();

        app.input = str_to_vec_char("My text");
        app.input_idx = 3;
        app.input_cursor_position = 3;

        handler(Key::Delete, &mut app);
        assert_eq!(app.input, str_to_vec_char("My ext"));

        app.input = str_to_vec_char("ラスト");
        app.input_idx = 1;
        app.input_cursor_position = 1;

        handler(Key::Delete, &mut app);
        assert_eq!(app.input, str_to_vec_char("ラト"));

        app.input = str_to_vec_char("Rust");
        app.input_idx = 2;
        app.input_cursor_position = 2;

        handler(Key::Ctrl('d'), &mut app);
        assert_eq!(app.input, str_to_vec_char("Rut"));
    }

    #[test]
    fn test_input_handler_left_event() {
        let mut app = App::default();

        app.input = str_to_vec_char("My text");
        let input_len = app.input.len().try_into().unwrap();
        app.input_idx = app.input.len();
        app.input_cursor_position = input_len;

        handler(Key::Left, &mut app);
        assert_eq!(app.input_cursor_position, input_len - 1);
        handler(Key::Left, &mut app);
        assert_eq!(app.input_cursor_position, input_len - 2);
        handler(Key::Left, &mut app);
        assert_eq!(app.input_cursor_position, input_len - 3);
        handler(Key::Ctrl('b'), &mut app);
        assert_eq!(app.input_cursor_position, input_len - 4);
        handler(Key::Ctrl('b'), &mut app);
        assert_eq!(app.input_cursor_position, input_len - 5);

        // Pretend to smash the left event to test the we have no out-of-bounds crash
        for _ in 0..20 {
            handler(Key::Left, &mut app);
        }

        assert_eq!(app.input_cursor_position, 0);
    }

    #[test]
    fn test_input_handler_on_enter_text_non_english_char() {
        let mut app = App::default();

        app.input = str_to_vec_char("ыа");
        app.input_cursor_position = app.input.len().try_into().unwrap();
        app.input_idx = app.input.len();

        handler(Key::Char('ы'), &mut app);

        assert_eq!(app.input, str_to_vec_char("ыаы"));
    }

    #[test]
    fn test_input_handler_on_enter_text_wide_char() {
        let mut app = App::default();

        app.input = str_to_vec_char("你");
        app.input_cursor_position = 2; // 你 is 2 char wide
        app.input_idx = 1; // 1 char

        handler(Key::Char('好'), &mut app);

        assert_eq!(app.input, str_to_vec_char("你好"));
        assert_eq!(app.input_idx, 2);
        assert_eq!(app.input_cursor_position, 4);
    }

    mod test_uri_parsing {
        use super::*;

        const URI_BASE: &str = "etrade:";
        const URL_BASE: &str = "https://open.etrade.com/";

        fn check_uri_parse(expected_id: &str, parsed: (String, bool)) {
            assert_eq!(parsed.1, true);
            assert_eq!(parsed.0, expected_id);
        }

        fn run_test_for_id_and_resource_type(id: &str, resource_type: &str) {
            check_uri_parse(
                id,
                etrade_resource_id(
                    URI_BASE,
                    &format!("etrade:{}:{}", resource_type, id),
                    ":",
                    resource_type,
                    ),
                    );
            check_uri_parse(
                id,
                etrade_resource_id(
                    URL_BASE,
                    &format!("https://open.etrade.com/{}/{}", resource_type, id),
                    "/",
                    resource_type,
                    ),
                    )
        }
    }
}

