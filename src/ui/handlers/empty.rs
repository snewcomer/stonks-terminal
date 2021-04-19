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
      | ActiveBlock::TickerDetail => {
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

  #[test]
  fn on_down_press() {
    let mut app = App::default();

    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Portfolio));

    handler(Key::Down, &mut app);
    let current_route = app.get_current_route();

    assert_eq!(current_route.active_block, ActiveBlock::Empty);
    assert_eq!(current_route.hovered_block, ActiveBlock::MyPlaylists);

    // TODO: test the other cases when they are implemented
  }

  #[test]
  fn on_up_press() {
    let mut app = App::default();

    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::MyPlaylists));

    handler(Key::Up, &mut app);
    let current_route = app.get_current_route();

    assert_eq!(current_route.active_block, ActiveBlock::Empty);
    assert_eq!(current_route.hovered_block, ActiveBlock::Portfolio);
  }

  #[test]
  fn on_left_press() {
    let mut app = App::default();
    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::AlbumTracks));

    handler(Key::Left, &mut app);
    let current_route = app.get_current_route();
    assert_eq!(current_route.active_block, ActiveBlock::Empty);
    assert_eq!(current_route.hovered_block, ActiveBlock::Portfolio);

    app.set_current_route_state(None, Some(ActiveBlock::Home));
    handler(Key::Left, &mut app);
    let current_route = app.get_current_route();
    assert_eq!(current_route.hovered_block, ActiveBlock::Portfolio);

    app.set_current_route_state(None, Some(ActiveBlock::TrackTable));
    handler(Key::Left, &mut app);
    let current_route = app.get_current_route();
    assert_eq!(current_route.hovered_block, ActiveBlock::Portfolio);
  }

  #[test]
  fn on_right_press() {
    let mut app = App::default();

    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Portfolio));
    app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
    handler(Key::Right, &mut app);
    let current_route = app.get_current_route();

    assert_eq!(current_route.active_block, ActiveBlock::AlbumTracks);
    assert_eq!(current_route.hovered_block, ActiveBlock::AlbumTracks);

    app.push_navigation_stack(RouteId::Search, ActiveBlock::Empty);
    app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
    handler(Key::Right, &mut app);
    let current_route = app.get_current_route();

    assert_eq!(current_route.active_block, ActiveBlock::SearchResultBlock);
    assert_eq!(current_route.hovered_block, ActiveBlock::SearchResultBlock);

    app.set_current_route_state(None, Some(ActiveBlock::Portfolio));
    app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
    handler(Key::Right, &mut app);
    let current_route = app.get_current_route();

    assert_eq!(current_route.active_block, ActiveBlock::TrackTable);
    assert_eq!(current_route.hovered_block, ActiveBlock::TrackTable);

    app.set_current_route_state(None, Some(ActiveBlock::Portfolio));
    app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
    handler(Key::Right, &mut app);
    let current_route = app.get_current_route();
    assert_eq!(current_route.active_block, ActiveBlock::TrackTable);
    assert_eq!(current_route.hovered_block, ActiveBlock::TrackTable);

    app.push_navigation_stack(RouteId::Home, ActiveBlock::Home);
    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Portfolio));
    handler(Key::Right, &mut app);
    let current_route = app.get_current_route();
    assert_eq!(current_route.active_block, ActiveBlock::Home);
    assert_eq!(current_route.hovered_block, ActiveBlock::Home);
  }
}

