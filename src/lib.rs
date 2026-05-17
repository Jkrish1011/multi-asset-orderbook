mod error;
mod types;

use std::{
    sync::{
        Arc,
    },
};

use crate::types::{OrderBook, Order, OrderType, Side};


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_orderbook() {
        let mut orderbook = OrderBook::new();   
        let order_id: u64 = 1;

        let order_1 = Order {
            order_type: OrderType::GoodTillCancel,
            order_id: order_id,
            side: Side::Buy,
            price: 100,
            initial_quantity: 100,
            remaining_quantity: 100
        };

        let order_2 = Order {
            order_type: OrderType::GoodTillCancel,
            order_id: order_id,
            side: Side::Buy,
            price: 200,
            initial_quantity: 10,
            remaining_quantity: 10
        };

        orderbook.add_order(order_1);
        orderbook.add_order(order_2);
        let size_of_orderbook = orderbook.size();
        println!("Size: {}", size_of_orderbook);
        assert_eq!(orderbook.size(), 1);
    }
}

