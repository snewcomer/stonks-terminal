use crate::app::{ActiveBlock, App, OrderFormState, RouteId};
use super::super::key::Key;

pub fn down_event(key: Key) -> bool {
    matches!(key, Key::Down | Key::Char('j') | Key::Ctrl('n'))
}

pub fn up_event(key: Key) -> bool {
    matches!(key, Key::Up | Key::Char('k') | Key::Ctrl('p'))
}

pub fn left_event(key: Key) -> bool {
    matches!(key, Key::Left | Key::Char('h') | Key::Ctrl('b'))
}

pub fn right_event(key: Key) -> bool {
    matches!(key, Key::Right | Key::Char('l') | Key::Ctrl('f'))
}

pub fn high_event(key: Key) -> bool {
    matches!(key, Key::Char('H'))
}

pub fn middle_event(key: Key) -> bool {
    matches!(key, Key::Char('M'))
}

pub fn low_event(key: Key) -> bool {
    matches!(key, Key::Char('L'))
}

pub fn on_down_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                let next_index = selection_index + 1;
                if next_index > selection_data.len() - 1 {
                    return 0;
                } else {
                    return next_index;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn on_up_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                if selection_index > 0 {
                    return selection_index - 1;
                } else {
                    return selection_data.len() - 1;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn on_high_press_handler() -> usize {
    0
}

pub fn on_middle_press_handler<T>(selection_data: &[T]) -> usize {
    let mut index = selection_data.len() / 2;
    if selection_data.len() % 2 == 0 {
        index -= 1;
    }
    index
}

pub fn on_low_press_handler<T>(selection_data: &[T]) -> usize {
    selection_data.len() - 1
}

pub fn handle_right_event(app: &mut App) {
    match app.get_current_route().hovered_block {
        ActiveBlock::Portfolio | ActiveBlock::WatchList => match app.get_current_route().id {
            RouteId::Search => {
                app.set_current_route_state(
                    None,
                    Some(ActiveBlock::SearchResults),
                );
            }
            RouteId::Home => {
                app.set_current_route_state(None, Some(ActiveBlock::Home));
            }
            RouteId::TickerDetail => {
                app.set_current_route_state(None, Some(ActiveBlock::TickerDetail));
            }
            RouteId::OrderForm => {
                if let OrderFormState::Quantity = app.order_form_state {
                    app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
                } else {
                    app.set_current_route_state(None, Some(ActiveBlock::OrderForm));
                }
            }
            RouteId::Error => {}
            RouteId::Analysis => {}
            _ => {}
        },
        _ => {}
    };
}

pub fn handle_left_event(app: &mut App) {
    // TODO: This should send you back to either library or playlist based on last selection
    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Portfolio));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_down_press_handler() {
        let data = vec!["Choice 1", "Choice 2", "Choice 3"];

        let index = 0;
        let next_index = on_down_press_handler(&data, Some(index));

        assert_eq!(next_index, 1);

        // Selection wrap if on last item
        let index = data.len() - 1;
        let next_index = on_down_press_handler(&data, Some(index));
        assert_eq!(next_index, 0);
    }

    #[test]
    fn test_on_up_press_handler() {
        let data = vec!["Choice 1", "Choice 2", "Choice 3"];

        let index = data.len() - 1;
        let next_index = on_up_press_handler(&data, Some(index));

        assert_eq!(next_index, index - 1);

        // Selection wrap if on first item
        let index = 0;
        let next_index = on_up_press_handler(&data, Some(index));
        assert_eq!(next_index, data.len() - 1);
    }
}

