use anyhow::{Result};

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
