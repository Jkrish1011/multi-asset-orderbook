
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

type Price = i32;
type Quantity = u32;
type OrderId = u64;
type LevelInfos = Vec<LevelInfo>;

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
