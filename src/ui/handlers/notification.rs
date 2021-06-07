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
            match &app.notifications {
                Some(p) => {
                    let next_index =
                        common_key_events::on_down_press_handler(&p, Some(app.selected_notification_index));
                    app.selected_notification_index = next_index;
                }
                None => {}
            };
        }
        k if common_key_events::up_event(k) => {
            match &app.notifications {
                Some(p) => {
                    let next_index =
                        common_key_events::on_up_press_handler(&p, Some(app.selected_notification_index));
                    app.selected_notification_index = next_index;
                }
                None => {}
            };
        }
        k if common_key_events::high_event(k) => {
            match &app.notifications {
                Some(_p) => {
                    let next_index = common_key_events::on_high_press_handler();
                    app.selected_notification_index = next_index;
                }
                None => {}
            };
        }
        k if common_key_events::middle_event(k) => {
            match &app.notifications {
                Some(p) => {
                    let next_index = common_key_events::on_middle_press_handler(&p);
                    app.selected_notification_index = next_index;
                }
                None => {}
            };
        }
        k if common_key_events::low_event(k) => {
            match &app.notifications {
                Some(p) => {
                    let next_index = common_key_events::on_low_press_handler(&p);
                    app.selected_notification_index = next_index;
                }
                None => {}
            };
        }
        Key::Enter => {
            if let (Some(notifications), selected_notification_index) =
                (&app.notifications, &app.selected_notification_index)
                {
                    if let Some(selected_notification) = notifications.get(selected_notification_index.to_owned()) {
                        let notification_id = selected_notification.id.to_owned();
                        app.dispatch(IoEvent::GetNotification(notification_id));
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
