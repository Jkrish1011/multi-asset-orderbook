mod error;
mod types;

use std::sync::Arc;

use crate::types::{Order, OrderBook, OrderType, Side};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_orderbook() {
        let mut orderbook = OrderBook::new();
        let order_id: u64 = 1;

        let order_1 = Order::new(OrderType::GoodTillCancel, order_id, Side::Buy, 100, 100);
        let order_2 = Order::new(OrderType::GoodTillCancel, order_id + 1, Side::Buy, 100, 200);
        let order_3 = Order::new(OrderType::FillAndKill, order_id + 2, Side::Sell, 200, 300);

        orderbook.add_order(order_1);
        orderbook.add_order(order_2);
        orderbook.add_order(order_3);

        let size_of_orderbook = orderbook.size();
        println!("Size: {}", size_of_orderbook);

        let order_info = orderbook.get_order_infos();
        println!("order_info: {:?}", order_info);

        assert_eq!(size_of_orderbook, 2);
    }
}
