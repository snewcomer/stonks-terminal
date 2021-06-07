use super::{
    super::super::app::{App, MAJOR_INDICES},
    common_key_events,
};
use super::super::key::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &MAJOR_INDICES,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        k if common_key_events::up_event(k) => {
            let next_index =
                common_key_events::on_up_press_handler(&MAJOR_INDICES, Some(app.library.selected_index));
            app.library.selected_index = next_index;
        }
        k if common_key_events::high_event(k) => {
            let next_index = common_key_events::on_high_press_handler();
            app.library.selected_index = next_index;
        }
        k if common_key_events::middle_event(k) => {
            let next_index = common_key_events::on_middle_press_handler(&MAJOR_INDICES);
            app.library.selected_index = next_index;
        }
        k if common_key_events::low_event(k) => {
            let next_index = common_key_events::on_low_press_handler(&MAJOR_INDICES);
            app.library.selected_index = next_index
        }
        // `library` should probably be an array of structs with enums rather than just using indexes
        // like this
        Key::Enter => match app.library.selected_index {
            // Nasdaq
            0 => {
                app.dispatch(IoEvent::GetNasdaq);
            }
            // DJIA
            1 => {
                app.dispatch(IoEvent::GetDowJones);
            }
            // S&P
            2 => {
                app.dispatch(IoEvent::GetSandP);
            }
            // This is required because Rust can't tell if this pattern in exhaustive
            _ => {}
        },
        _ => (),
    };
}

