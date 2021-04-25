use super::{
    super::super::app::{ActiveBlock, App, OrderFormState, RouteId},
    super::key::Key,
    common_key_events,
};
use crate::clients::etrade_json_structs::{OrderAction, OrderType};

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('B') => {
            // BUY
            app.new_preview_order(OrderType::EQ, OrderAction::BUY);
            app.order_form_state = OrderFormState::Quantity;
            let symbol = app.selected_ticker.as_ref().unwrap().ticker.symbol.to_owned();
            app.preview_order_ticker = Some(symbol.to_owned());
            app.add_next_order_field("symbol", symbol.to_owned());

            // Dbl push so Esc "back" works
            app.push_navigation_stack(RouteId::OrderForm, ActiveBlock::OrderForm);
            app.push_navigation_stack(RouteId::OrderForm, ActiveBlock::Input);
        }
        Key::Char('S') => {
            // SELL
            app.new_preview_order(OrderType::EQ, OrderAction::SELL);
            app.order_form_state = OrderFormState::Quantity;
            let symbol = app.selected_ticker.as_ref().unwrap().ticker.symbol.to_owned();
            app.preview_order_ticker = Some(symbol.to_owned());
            app.add_next_order_field("symbol", symbol.to_owned());

            // Dbl push so Esc "back" works
            app.push_navigation_stack(RouteId::OrderForm, ActiveBlock::OrderForm);
            app.push_navigation_stack(RouteId::OrderForm, ActiveBlock::Input);
        }
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
