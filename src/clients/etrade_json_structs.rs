use serde::{Deserialize, Serialize};

// ORDER
#[derive(strum_macros::ToString, strum_macros::EnumString, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum OrderType {
    EQ,
    OPTN,
    SPREADS,
    BUY_WRITES,
    BUTTERFLY,
    IRON_BUTTERFLY,
    CONDOR,
    IRON_CONDOR,
    MF,
    MMF
}

#[derive(strum_macros::ToString, Clone, Debug, strum_macros::EnumString, Deserialize, Serialize, PartialEq)]
pub enum OrderAction {
    BUY,
    SELL,
    BUY_TO_COVER,
    SELL_SHORT,
    BUY_OPEN,
    BUY_CLOSE,
    SELL_OPEN,
    SELL_CLOSE,
    EXCHANGE
}

#[derive(strum_macros::ToString, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum StatusType {
    OPEN, EXECUTED, CANCELLED, INDIVIDUAL_FILLS, CANCEL_REQUESTED, EXPIRED, REJECTED, PARTIAL, DO_NOT_EXERCISE, DONE_TRADE_EXECUTED
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub symbol: String,
    pub security_type: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub symbol_description: Option<String>,
    pub order_action: String,
    pub quantity: String,
    pub quantity_type: String,
    pub cancel_quantity: Option<String>,
    pub reserve_order: Option<bool>,
    pub product: Product,
}

// https://apisb.etrade.com/docs/api/order/api-order-v1.html#/definitions/OrderDetail
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub all_or_none: bool,
    pub account_id: String,
    pub placed_time: String,
    pub executed_time: String,
    pub status: String,
    pub order_number: String,
    pub order_term: String,
    pub market_session: String,
    pub preview_id: String,
    pub price_type: String,
    pub price_value: String,
    pub limit_price: String,
    pub stop_price: String,
    pub net_price: String,
    pub net_bid: String,
    pub net_ask: String,
    pub estimated_total_amount: String,
    pub estimated_commission: String,
    pub instrument: Vec<Instrument>,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            all_or_none: true,
            account_id: "".to_string(),
            placed_time: "".to_string(),
            executed_time: "".to_string(),
            status: "".to_string(),
            order_number: "".to_string(),
            order_term: "".to_string(),
            market_session: "".to_string(),
            preview_id: "".to_string(),
            price_type: "".to_string(),
            price_value: "".to_string(),
            limit_price: "".to_string(),
            stop_price: "".to_string(),
            net_price: "".to_string(),
            net_bid: "".to_string(),
            net_ask: "".to_string(),
            estimated_total_amount: "".to_string(),
            estimated_commission: "".to_string(),
            instrument: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreviewId {
    pub preview_id: String,
}

// #[derive(Debug, Deserialize, Serialize, PartialEq)]
// #[serde(rename_all = "camelCase")]
// pub struct OrderRequest {
//     pub order_id: String,
//     pub client_order_id: String,
//     pub order: Vec<Order>,
//     pub preview_ids: Vec<PreviewId>,
// }


// https://apisb.etrade.com/docs/api/order/api-order-v1.html#/definitions/PlaceOrderRequest
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreviewOrderRequest {
    pub order_type: String,
    pub client_order_id: String,
    pub order: Vec<Order>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreviewOrderResponse {
    pub order_type: String,
    pub account_id: String,
    pub client_order_id: String,
    pub preview_time: String,
    pub order: Vec<Order>,
    pub preview_ids: Vec<PreviewId>,
    pub total_commission: String,
    pub commission_message: String,
    pub total_order_value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub order_type: OrderType,
    pub client_order_id: String,
    pub order: Vec<Order>,
    pub preview_ids: Vec<PreviewId>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_type: OrderType,
    pub account_id: String,
    pub client_order_id: String,
    pub preview_time: String,
    pub message_list: Vec<String>,
    pub order: Vec<Order>,
    pub order_id: String,
    pub preview_ids: Vec<PreviewId>,
    pub total_commission: String,
    pub commission_message: String,
    pub total_order_value: String,
}
