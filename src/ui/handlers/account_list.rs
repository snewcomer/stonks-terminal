use super::{
    super::super::app::App,
    super::key::Key,
    common_key_events,
};

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        k if common_key_events::down_event(k) => {
            match &app.user_accounts {
                Some(p) => {
                    if let Some(selected_account_index) = app.selected_account_index {
                        let next_index =
                            common_key_events::on_down_press_handler(&p, Some(selected_account_index));
                        app.selected_account_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        k if common_key_events::up_event(k) => {
            match &app.user_accounts {
                Some(p) => {
                    let next_index =
                        common_key_events::on_up_press_handler(&p, app.selected_account_index);
                    app.selected_account_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::high_event(k) => {
            match &app.user_accounts {
                Some(_p) => {
                    let next_index = common_key_events::on_high_press_handler();
                    app.selected_account_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::middle_event(k) => {
            match &app.user_accounts {
                Some(p) => {
                    let next_index = common_key_events::on_middle_press_handler(&p);
                    app.selected_account_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::low_event(k) => {
            match &app.user_accounts {
                Some(p) => {
                    let next_index = common_key_events::on_low_press_handler(&p);
                    app.selected_account_index = Some(next_index);
                }
                None => {}
            };
        }
        Key::Enter => {
            if let Some(selected_account_index) = &app.selected_account_index {
                app.active_account_index = Some(*selected_account_index);
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
