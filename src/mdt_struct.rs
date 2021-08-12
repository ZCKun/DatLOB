#[derive(Copy, Clone, Debug)]
#[repr(C, align(8))]
pub struct SzL2Order {
    pub(crate) rec_time: u64,  // 8
    pub(crate) channel_no: u32, // 4
    pub(crate) order_id: u64,  // 8
    pub(crate) symbol_code: [u8; 40],  // 40
    pub(crate) symbol_source: [u8; 5],  // 5
    pub(crate) time: i64,  // 8
    pub(crate) price: f64,  // 8
    pub(crate) qty: f64,  // 8
    pub(crate) code: u8,  // 1
    pub(crate) r#type: u8  // 1
}

#[repr(C, align(8))]
pub struct SzL2Trans {
    pub(crate) rec_time: u64,
    pub(crate) trade_id: u32,
    pub(crate) bid_order_id: i64,
    pub(crate) ask_order_id: i64,
    pub(crate) symbol_code: [u8; 40],
    pub(crate) symbol_source: [u8; 5],
    pub(crate) trade_time: i32,
    pub(crate) price: f64,
    pub(crate) qty: f64,
    pub(crate) trade_flag: u8,
}

#[repr(C, align(8))]
pub struct SzBuySellLevelInfo3 {
    pub(crate) price: f64,
    pub(crate) volume: f64,
    pub(crate) total_order_no: u64,
}

#[repr(C, align(8))]
pub struct Szl2Quotation {
    pub(crate) qds_time: u64,
    pub(crate) time: i64,
    pub(crate) symbol: [u8; 40],
    pub(crate) symbol_source: [u8; 5],
    pub(crate) pre_close_price: f64,
    pub(crate) open_price: f64,
    pub(crate) last_price: f64,
    pub(crate) high_price: f64,
    pub(crate) low_price: f64,
    pub(crate) price_up_limit: f64,
    pub(crate) price_down_limit: f64,
    pub(crate) price_up_down_1: f64,
    pub(crate) price_up_down_2: f64,
    pub(crate) total_no: u64,
    pub(crate) total_volume: f64,
    pub(crate) total_amount: f64,
    pub(crate) close_price: f64,
    pub(crate) security_phase_tag: [u8; 8],
    pub(crate) pe_ratio_1: f64,
    pub(crate) nav: f64,
    pub(crate) pe_ratio_2: f64,
    pub(crate) iopv: f64,
    pub(crate) premium_rate: f64,
    pub(crate) total_sell_order_volume: f64,
    pub(crate) wt_avg_sell_price: f64,
    pub(crate) sell_level_no: u32,
    pub(crate) sell_level: [SzBuySellLevelInfo3; 10],
    pub(crate) sell_price_01: f64,
    pub(crate) sell_volume_01: f64,
    pub(crate) total_sell_order_no_01: u64,
    pub(crate) sell_price_02: f64,
    pub(crate) sell_volume_02: f64,
    pub(crate) total_sell_order_no_02: u64,
    pub(crate) sell_price_03: f64,
    pub(crate) sell_volume_03: f64,
    pub(crate) total_sell_order_no_03: u64,
    pub(crate) sell_price_04: f64,
    pub(crate) sell_volume_04: f64,
    pub(crate) total_sell_order_no_04: u64,
    pub(crate) sell_price_05: f64,
    pub(crate) sell_volume_05: f64,
    pub(crate) total_sell_order_no_05: u64,
    pub(crate) sell_price_06: f64,
    pub(crate) sell_volume_06: f64,
    pub(crate) total_sell_order_no_06: u64,
    pub(crate) sell_price_07: f64,
    pub(crate) sell_volume_07: f64,
    pub(crate) total_sell_order_no_07: u64,
    pub(crate) sell_price_08: f64,
    pub(crate) sell_volume_08: f64,
    pub(crate) total_sell_order_no_08: u64,
    pub(crate) sell_price_09: f64,
    pub(crate) sell_volume_09: f64,
    pub(crate) total_sell_order_no_09: u64,
    pub(crate) sell_price_10: f64,
    pub(crate) sell_volume_10: f64,
    pub(crate) total_sell_order_no_10: u64,
    pub(crate) sell_level_queue_no_01: u32,
    pub(crate) sell_level_queue: [f64; 50],

    pub(crate) total_buy_order_volume: f64,
    pub(crate) wt_avg_buy_price: f64,
    pub(crate) buy_level_no: u32,
    pub(crate) buy_level: [SzBuySellLevelInfo3; 10],
    pub(crate) buy_price_01: f64,
    pub(crate) buy_volume_01: f64,
    pub(crate) total_buy_order_no_01: u64,
    pub(crate) buy_price_02: f64,
    pub(crate) buy_volume_02: f64,
    pub(crate) total_buy_order_no_02: u64,
    pub(crate) buy_price_03: f64,
    pub(crate) buy_volume_03: f64,
    pub(crate) total_buy_order_no_03: u64,
    pub(crate) buy_price_04: f64,
    pub(crate) buy_volume_04: f64,
    pub(crate) total_buy_order_no_04: u64,
    pub(crate) buy_price_05: f64,
    pub(crate) buy_volume_05: f64,
    pub(crate) total_buy_order_no_05: u64,
    pub(crate) buy_price_06: f64,
    pub(crate) buy_volume_06: f64,
    pub(crate) total_buy_order_no_06: u64,
    pub(crate) buy_price_07: f64,
    pub(crate) buy_volume_07: f64,
    pub(crate) total_buy_order_no_07: u64,
    pub(crate) buy_price_08: f64,
    pub(crate) buy_volume_08: f64,
    pub(crate) total_buy_order_no_08: u64,
    pub(crate) buy_price_09: f64,
    pub(crate) buy_volume_09: f64,
    pub(crate) total_buy_order_no_09: u64,
    pub(crate) buy_price_10: f64,
    pub(crate) buy_volume_10: f64,
    pub(crate) total_buy_order_no_10: u64,
    pub(crate) buy_level_queue_no_01: u32,
    pub(crate) buy_level_queue: [f64; 50],

    pub(crate) wt_avg_rate: f64,
    pub(crate) wt_avg_rate_up_down: f64,
    pub(crate) pre_wt_avg_rate: f64,
}

