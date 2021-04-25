use super::{
    super::super::app::{ActiveBlock, App, DialogContext},
    super::key::Key,
};

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Enter => {
            if let Some(route) = app.pop_navigation_stack() {
                if app.confirm {
                    if let ActiveBlock::Dialog(d) = route.active_block {
                        match d {
                            DialogContext::TickerDetail => handle_ticker_detail(app),
                        }
                    }
                }
            }
        }
        Key::Char('q') => {
            app.pop_navigation_stack();
        }
        Key::Right => app.confirm = !app.confirm,
        Key::Left => app.confirm = !app.confirm,
        _ => {}
    }
}

fn handle_ticker_detail(app: &mut App) {
    // transition to order page
    todo!();
}
