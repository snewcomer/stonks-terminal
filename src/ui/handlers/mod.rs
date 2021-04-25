mod common_key_events;
mod dialog;
mod empty;
mod portfolio;
mod input;
mod order_form;
mod watch_list;
mod account_list;
mod search_results;
mod ticker_detail;

use crate::app::{ActiveBlock, App, RouteId, OrderFormState};
use super::key::Key;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        // _ if key == app.user_config.keys.help => {
        //   app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        // }
        _ if key == app.user_config.keys.search => {
          app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        // _ if key == app.user_config.keys.basic_view => {
        //   app.push_navigation_stack(RouteId::BasicView, ActiveBlock::BasicView);
        // }
        _ => handle_block_events(key, app),
    }
}

// Handle event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.get_current_route();
    match current_route.active_block {
        ActiveBlock::Input => {
            input::handler(key, app);
        }
        // ActiveBlock::HelpMenu => {
        //     help_menu::handler(key, app);
        // }
        // ActiveBlock::Error => {
        //     error_screen::handler(key, app);
        // }
        ActiveBlock::Dialog(_) => {
            dialog::handler(key, app);
        }
        ActiveBlock::SearchResults => {
            search_results::handler(key, app);
        }
        // ActiveBlock::Home => {
        //     home::handler(key, app);
        // }
        ActiveBlock::WatchList => {
            watch_list::handler(key, app);
        }
        ActiveBlock::TickerDetail => {
            ticker_detail::handler(key, app);
        }
        ActiveBlock::Portfolio => {
            portfolio::handler(key, app);
        }
        ActiveBlock::AccountList => {
            account_list::handler(key, app);
        }
        ActiveBlock::OrderForm => {
            order_form::handler(key, app);
        }
        ActiveBlock::Empty => {
            empty::handler(key, app);
        }
    //     ActiveBlock::RecentlyPlayed => {
    //         recently_played::handler(key, app);
    //     }
    //     ActiveBlock::BasicView => {
    //         basic_view::handler(key, app);
    //     }
    //     ActiveBlock::Dialog(_) => {
    //         dialog::handler(key, app);
    //     }
         _ => {}
    }
}

fn handle_escape(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::SearchResults => {
            app.search_results.tickers = None;
            app.search_results.selected_ticker_index = None;
            app.pop_navigation_stack();
        }
        ActiveBlock::TickerDetail => {
            if let (Some(_tickers), Some(_selected_ticker_index)) =
                (&app.search_results.tickers, &app.search_results.selected_ticker_index)
                {
                    app.push_navigation_stack(RouteId::Search, ActiveBlock::SearchResults);
                    app.selected_ticker = None;
                } else {
                    app.pop_navigation_stack();
                }
        }
        ActiveBlock::OrderForm => {
            match app.order_form_state {
                OrderFormState::Submit => {
                    app.order_form_state = OrderFormState::Quantity;
                    app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
                }
                _ => ()
            }
            app.pop_navigation_stack();
        }
        ActiveBlock::Dialog(_) => {
            app.pop_navigation_stack();
        }
        ActiveBlock::Error => {
            app.pop_navigation_stack();
        }
        _ => {
            // if let OrderFormState::Quantity = app.order_form_state {
            //     app.order_form_state = OrderFormState::Initial;
            //     app.cancel_preview_order();
            //     app.push_navigation_stack(RouteId::TickerDetail, ActiveBlock::TickerDetail);
            // // if RouteId::OrderForm == app.get_current_route().id {
            // //     app.order_form_state = OrderFormState::Quantity;
            // //     app.pop_navigation_stack();
            // } else {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
            // }
        }
    }
}

// fn handle_jump_to_context(app: &mut App) {
//   if let Some(current_playback_context) = &app.current_playback_context {
//     if let Some(play_context) = current_playback_context.context.clone() {
//       match play_context._type {
//         rspotify::senum::Type::Album => handle_jump_to_album(app),
//         rspotify::senum::Type::Artist => handle_jump_to_artist_album(app),
//         rspotify::senum::Type::Playlist => {
//           app.dispatch(IoEvent::GetPlaylistTracks(play_context.uri, 0))
//         }
//         _ => {}
//       }
//     }
//   }
// }

// fn handle_jump_to_album(app: &mut App) {
//   if let Some(CurrentlyPlaybackContext {
//     item: Some(item), ..
//   }) = app.current_playback_context.to_owned()
//   {
//     match item {
//       PlayingItem::Track(track) => {
//         app.dispatch(IoEvent::GetAlbumTracks(Box::new(track.album)));
//       }
//       PlayingItem::Episode(episode) => {
//         app.dispatch(IoEvent::GetShowEpisodes(Box::new(episode.show)));
//       }
//     };
//   }
// }

// // NOTE: this only finds the first artist of the song and jumps to their albums
// fn handle_jump_to_artist_album(app: &mut App) {
//   if let Some(CurrentlyPlaybackContext {
//     item: Some(item), ..
//   }) = app.current_playback_context.to_owned()
//   {
//     match item {
//       PlayingItem::Track(track) => {
//         if let Some(artist) = track.artists.first() {
//           if let Some(artist_id) = artist.id.clone() {
//             app.get_artist(artist_id, artist.name.clone());
//             app.push_navigation_stack(RouteId::Artist, ActiveBlock::ArtistBlock);
//           }
//         }
//       }
//       PlayingItem::Episode(_episode) => {
//         // Do nothing for episode (yet!)
//       }
//     }
//   };
// }

