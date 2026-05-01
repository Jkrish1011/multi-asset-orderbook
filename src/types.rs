use anyhow::{Result};
use std::{
    sync::{
        Arc,
    },
    collections::{VecDeque},
};

use crate::error::CustomError;

#[derive(Debug, Copy, Clone)]
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
}

/*
    When we are writing to an ordebook for a new order, we need all the fields.
    When we are modifying the order, we need the order_id generated before and the new quantity
    When we are cancelling, we need the order_id only.
*/


type OrderPointer = Arc<Order>;
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
        Arc::new(Order::new(order_type, self.order_id, self.side, self.price, self.quantity))
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



pub struct OrderBook {
    pub order_book: Arc<Vec<Order>>,
}

impl OrderBook {

    pub fn new() -> Self {
        let vec_orders: Vec<Order> = Vec::new();
        
        Self {
            order_book: Arc::new(vec_orders)
        }
    }

    pub fn run(&self) -> Result<(), CustomError> {
        
        return Ok(());
    }
}