use super::common_key_events;
use crate::{
  app::{ActiveBlock, App},
  ui::Key,
};

// When no block is actively selected, just handle regular event
pub fn handler(key: Key, app: &mut App) {
  match key {
    Key::Enter => {
      let current_hovered = app.get_current_route().hovered_block;
      app.set_current_route_state(Some(current_hovered), None);
    }
    k if common_key_events::down_event(k) => match app.get_current_route().hovered_block {
      ActiveBlock::WatchList => {
        app.selected_watch_list_index = Some(0);
        app.set_current_route_state(None, Some(ActiveBlock::Portfolio));
      }
      _ => {}
    },
    k if common_key_events::up_event(k) => match app.get_current_route().hovered_block {
      ActiveBlock::Portfolio => {
        app.set_current_route_state(None, Some(ActiveBlock::WatchList));
      }
      _ => {}
    },
    k if common_key_events::left_event(k) => match app.get_current_route().hovered_block {
      ActiveBlock::Home
      | ActiveBlock::TickerDetail
      | ActiveBlock::OrderForm => {
        app.set_current_route_state(None, Some(ActiveBlock::WatchList));
      }
      _ => {}
    },
    k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
    _ => (),
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::app::RouteId;

  #[test]
  fn on_enter() {
    let mut app = App::default();

    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Portfolio));

    handler(Key::Enter, &mut app);
    let current_route = app.get_current_route();

    assert_eq!(current_route.active_block, ActiveBlock::Portfolio);
    assert_eq!(current_route.hovered_block, ActiveBlock::Portfolio);
  }
}

