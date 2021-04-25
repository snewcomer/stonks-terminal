extern crate unicode_width;

use super::super::super::app::{App, OrderFormState};
use crate::ui::key::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Enter => {
            process_order(app);
        }
        _ => {}
    }
}

fn process_order(app: &mut App) {
    if let OrderFormState::Submit = app.order_form_state {
        app.dispatch(IoEvent::SubmitPreviewRequest);
    }
}
