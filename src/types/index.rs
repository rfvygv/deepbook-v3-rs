#[derive(Debug, Clone)]
pub struct BalanceManager {
    pub address: String,
    pub trade_cap: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Coin {
    pub address: String,
    pub coin_type: String,
    pub scalar: usize,
}

#[derive(Clone, Debug)]
pub struct Pool {
    pub address: String,
    pub base_coin: String,
    pub quote_coin: String,
}

// Trading constants
pub enum OrderType {
    NoRestriction,
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

// Self matching options
pub enum SelfMatchingOptions {
    SelfMatchingAllowed,
    CancelTaker,
    CancelMaker,
}

pub struct PlaceLimitOrderParams {
    pub pool_key: String,
    pub balance_manager_key: String,
    pub client_order_id: String,
    pub price: usize,
    pub quantity: usize,
    pub is_bid: bool,
    // expiration?: number | bigint;
    // orderType?: OrderType;
    // selfMatchingOption?: SelfMatchingOptions;
    // payWithDeep?: boolean;
}

pub struct PlaceMarketOrderParams {
    pub pool_key: String,
    pub balance_manager_key: String,
    pub client_order_id: String,
    pub quantity: usize,
    pub is_bid: bool,
    // pub selfMatchingOption?: SelfMatchingOptions,
    pub pay_with_deep: Option<bool>,
}

pub struct ProposalParams {
    pub pool_key: String,
    pub balance_manager_key: String,
    pub taker_fee: usize,
    pub maker_fee: usize,
    pub stake_required: usize,
}

pub struct SwapParams {
    pub pool_key: String,
    pub amount: usize,
    pub deep_amount: String,
    pub min_out: usize,
    // pub deep_coin: Option<TransactionObjectArgument>,
    // pub base_coin: Option<TransactionObjectArgument>,
    // pub quote_coin: Option<TransactionObjectArgument>,
}

pub struct CreatePoolAdminParams {
    pub base_coin_key: String,
    pub quote_coin_key: String,
    pub tick_size: usize,
    pub lot_size: usize,
    pub min_size: usize,
    pub whitelisted: bool,
    pub stable_pool: bool,
    // pub deep_coin: Option<TransactionObjectArgument>,
    // pub base_coin: Option<TransactionObjectArgument>,
}

pub struct Config {
    pub deepbook_package_id: String,
    pub registry_id: String,
    pub deep_treasury_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Mainnet,
    Testnet,
}
