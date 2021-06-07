use super::{
    super::super::app::{App},
    super::key::Key,
};
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('N') => {
            app.dispatch(IoEvent::GetNotifications);
            // Notifications
            // // Dbl push so Esc "back" works
            // app.push_navigation_stack(RouteId::Notifications, ActiveBlock::Notifications);
            // app.push_navigation_stack(RouteId::Notifications, ActiveBlock::Notifications);
        }
        // k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        // k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
