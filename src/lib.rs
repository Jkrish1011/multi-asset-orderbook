mod error;
mod types;

use std::{
    sync::{
        Arc,
    },
};

use crate::types::{Order, OrderType, OrderId, Side, Price, Quantity};
use crate::error::{CustomError};

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