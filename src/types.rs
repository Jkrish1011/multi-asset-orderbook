use anyhow::{Result};
use std::{
    sync::{
        Arc, Mutex
    },
    cmp,
    cmp::{Reverse},
    collections::{BTreeMap, VecDeque, HashMap},
};

use crate::error::CustomError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OrderType {
    GoodTillCancel,
    FillAndKill,
}

#[derive(Debug, Copy, Clone)]
pub enum Side {
    Buy,
    Sell,
}

pub type Price = i32;
pub type Quantity = u32;
pub type OrderId = u64;
pub type LevelInfos = Vec<LevelInfo>;

#[derive(Debug, Copy, Clone)]
pub struct LevelInfo {
    pub price: Price,
    pub quantity: Quantity,
}

#[derive(Debug, Clone)]
pub struct OrderbookLevelInfos {
    bids: Vec<LevelInfo>,
    asks: Vec<LevelInfo>,
}

impl OrderbookLevelInfos {
    pub fn new(bids: Vec<LevelInfo>, asks: Vec<LevelInfo>) -> Self {
        OrderbookLevelInfos {
            bids,
            asks
        }
    } 

    pub fn get_bids(&self) -> &Vec<LevelInfo> {
        &self.bids
    }

    pub fn get_asks(&self) -> &Vec<LevelInfo> {
        &self.asks
    }
}

pub struct Order {
    pub order_type: OrderType,
    pub order_id: OrderId,
    pub side: Side,
    pub price: Price, 
    pub initial_quantity: Quantity,
    pub remaining_quantity: Quantity,
}

impl Order {
    pub fn new(order_type: OrderType, order_id: OrderId, side: Side, price: Price, quantity: Quantity) -> Self {
        Self {
            order_type,
            order_id,
            side,
            price,
            initial_quantity: quantity,
            remaining_quantity: quantity
        }
    }

    pub fn get_order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn get_order_type(&self) -> OrderType {
        self.order_type
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn get_price(&self) -> Price {
        self.price
    }

    pub fn get_initial_quantity(&self) -> Quantity {
        self.initial_quantity
    }

    pub fn get_remaining_quantity(&self) -> Quantity {
        self.remaining_quantity
    }

    pub fn get_filled_quantity(&self) -> Quantity {
        self.initial_quantity - self.remaining_quantity
    }

    pub fn fill(&mut self, quantity: Quantity) -> Result<(), CustomError> {

        if quantity > self.remaining_quantity {
            return Err(CustomError::InvalidFillAmount(format!("Order ({}) : ({}) quantity cannot be filled for more than it's remaining quantity : ({})", self.order_id, quantity, self.remaining_quantity)));
        }

        self.remaining_quantity -= quantity;

        Ok(())
    }

    pub fn is_filled(&self) -> bool {
        return self.get_remaining_quantity() == 0;
    }
}

/*
    When we are writing to an ordebook for a new order, we need all the fields.
    When we are modifying the order, we need the order_id generated before and the new quantity
    When we are cancelling, we need the order_id only.
*/


type OrderPointer = Arc<Mutex<Order>>;
type OrderPointers = VecDeque<OrderPointer>;

pub struct OrderModify {
    pub order_id: OrderId,
    pub price: Price,
    pub side: Side,
    pub quantity: Quantity,
}

impl OrderModify {
    pub fn new(order_id: OrderId, side: Side, price: Price, quantity: Quantity) -> Self {
        Self {
            order_id,
            price,
            side, 
            quantity
        }
    }

    pub fn get_order_id(&self) -> OrderId { self.order_id }
    pub fn get_price(&self) -> Price { self.price }
    pub fn get_side(&self) -> Side { self.side }
    pub fn get_quantity(&self) -> Quantity { self.quantity }

    pub fn to_order_pointer(&self, order_type: OrderType) -> OrderPointer {
        Arc::new(Mutex::new(Order::new(order_type, self.order_id, self.side, self.price, self.quantity)))
    }
}

pub struct TradeInfo {
    pub order_id: OrderId,
    pub quantity: Quantity,
    pub price: Price,
}

pub struct Trade {
    pub bid_trade: TradeInfo,
    pub ask_trade: TradeInfo,
}

impl Trade {
    pub fn new(bid_trade: TradeInfo, ask_trade: TradeInfo) -> Self {
        Self {
            bid_trade,
            ask_trade,
        }
    }

    pub fn get_bid_trade(&self) -> &TradeInfo {
        &self.bid_trade
    }

    pub fn get_ask_trade(&self) -> &TradeInfo {
        &self.ask_trade
    }
}

type Trades = VecDeque<Trade>;

struct OrderEntry {
    order: OrderPointer,
    location: usize
}

pub struct OrderBook {
    pub bids: BTreeMap<Reverse<Price>, OrderPointers>,
    pub asks: BTreeMap<Price, OrderPointers>,
    pub orders: HashMap<OrderId, OrderEntry>,
}

impl OrderBook {
    fn can_match(&mut self, side: Side, price: Price) -> bool {
        match side {
            Side::Buy => {
                if self.asks.len() == 0 {
                    println!("Cannot match. No Asks");
                    return false;
                }

                let Some((&best_ask_price, _best_ask_order_p)) = self.asks.first_key_value() else {
                    return false;
                };
                return price >= best_ask_price;
            }
            Side::Sell => {
                if self.bids.len() == 0 {
                    println!("Cannot match. No Bids");
                    return false;
                }

                let Some((&best_bid_price, _best_bid_order_p)) = self.bids.first_key_value() else {
                    return false;
                };
                return price <= best_bid_price.0;
            }
        }
    }

    fn match_orders(&mut self) -> Trades {
        let mut trades: Trades = VecDeque::new();

        loop {
            let Some((&best_ask_price, _best_ask_order_p)) = self.asks.first_key_value() else {
                return trades;
            };
            
            let Some((&best_bid_price, _best_bid_order_p)) = self.bids.first_key_value() else {
                return trades;
            };

            if best_ask_price <= best_bid_price.0 {
                break;
            }

            while self.bids.len() > 0 && self.asks.len() > 0 {
                let Some((bid_price, bid_order_p)) = self.bids.pop_first() else {
                    continue;
                };

                let Some((ask_price, ask_order_p)) = self.asks.pop_first() else {
                    continue;
                };

                let Some(bid_remaining_qty) = bid_order_p.front() else{
                    continue;
                };

                let Some(ask_remaining_qty) = ask_order_p.front() else{
                    continue;
                };

                // lock and get access to the first values of each ask and bid.
                {
                    let mut ask_obj = ask_remaining_qty.lock().unwrap();
                    let mut bid_obj = bid_remaining_qty.lock().unwrap();

                    let qty: Quantity = cmp::min(ask_obj.get_remaining_quantity(), ask_obj.get_remaining_quantity());

                    let _ = ask_obj.fill(qty.clone());
                    let _ = bid_obj.fill(qty.clone());

                    if bid_obj.is_filled() {
                        let Some(_) = bid_order_p.front() else {
                            continue;
                        };
                    }
                    if ask_obj.is_filled() {
                        let Some(_) = ask_order_p.front() else {
                            continue;
                        };
                    }

                    let curr_bid_trade = TradeInfo {
                        order_id: bid_obj.get_order_id(),
                        quantity: qty.clone(), 
                        price: bid_obj.get_price(),
                    };

                    let curr_ask_trade = TradeInfo {
                        order_id: ask_obj.get_order_id(),
                        quantity: qty.clone(), 
                        price: ask_obj.get_price(),
                    };

                    let curr_trade = Trade::new(curr_bid_trade, curr_ask_trade);

                    trades.push_back(curr_trade);
                }

                if bid_order_p.len() == 0 {
                    self.bids.remove(&bid_price);
                }

                if ask_order_p.len() == 0 {
                    self.asks.remove(&ask_price);
                }
            }
        }

        // Check for fill and kill orders. If not matches found from above, just Cancel the fok order.
        if self.bids.len() == 0 {
            let Some((&_best_bid_price, best_bid_order_p)) = self.bids.first_key_value() else {
                return trades;
            };

            let Some(bid_remaining_qty) = best_bid_order_p.front() else {
                return trades;
            };

            {
                let mut bid_obj = bid_remaining_qty.lock().unwrap();
                if bid_obj.get_order_type() == OrderType::FillAndKill {
                    self.cancel_order(bid_obj.get_order_id());
                }
            }
        }

        if self.asks.len() == 0 {
            let Some((&_best_ask_price, best_ask_order_p)) = self.asks.first_key_value() else {
                return trades;
            };

            let Some(ask_remaining_qty) = best_ask_order_p.front() else {
                return trades;
            };

            {
                let mut ask_obj = ask_remaining_qty.lock().unwrap();
                if ask_obj.get_order_type() == OrderType::FillAndKill {
                    self.cancel_order(ask_obj.get_order_id());
                }
            }
        }
        
 
        trades
    }

    pub fn cancel_order(&self, order_id: OrderId) -> () {

    }
}