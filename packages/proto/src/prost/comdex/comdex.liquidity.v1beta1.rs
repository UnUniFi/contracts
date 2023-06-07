// @generated
/// Params defines the parameters for the liquidity module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
/// Params defines the parameters for the liquidity module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericParams {
    #[prost(uint64, tag = "1")]
    pub batch_size: u64,
    #[prost(uint64, tag = "2")]
    pub tick_precision: u64,
    #[prost(string, tag = "3")]
    pub fee_collector_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub dust_collector_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub min_initial_pool_coin_supply: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub pair_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "7")]
    pub pool_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "8")]
    pub min_initial_deposit_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub max_price_limit_ratio: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub max_order_lifespan: ::core::option::Option<::prost_types::Duration>,
    #[prost(string, tag = "11")]
    pub swap_fee_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub withdraw_fee_rate: ::prost::alloc::string::String,
    #[prost(uint64, tag = "13")]
    pub deposit_extra_gas: u64,
    #[prost(uint64, tag = "14")]
    pub withdraw_extra_gas: u64,
    #[prost(uint64, tag = "15")]
    pub order_extra_gas: u64,
    #[prost(string, tag = "16")]
    pub swap_fee_distr_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub swap_fee_burn_rate: ::prost::alloc::string::String,
    #[prost(uint64, tag = "18")]
    pub app_id: u64,
    #[prost(uint64, tag = "19")]
    pub max_num_market_making_order_ticks: u64,
    #[prost(uint64, tag = "20")]
    pub max_num_active_pools_per_pair: u64,
}
/// Pair defines a coin pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub base_coin_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_coin_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub escrow_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub last_order_id: u64,
    #[prost(string, tag = "6")]
    pub last_price: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub current_batch_id: u64,
    #[prost(string, tag = "8")]
    pub swap_fee_collector_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    pub app_id: u64,
}
/// Pool defines a basic liquidity pool with no min-price and max-price.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub pair_id: u64,
    #[prost(string, tag = "3")]
    pub reserve_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pool_coin_denom: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub last_deposit_request_id: u64,
    #[prost(uint64, tag = "6")]
    pub last_withdraw_request_id: u64,
    #[prost(bool, tag = "7")]
    pub disabled: bool,
    #[prost(uint64, tag = "8")]
    pub app_id: u64,
    #[prost(enumeration = "PoolType", tag = "9")]
    pub r#type: i32,
    #[prost(string, tag = "10")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub min_price: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub max_price: ::prost::alloc::string::String,
}
/// DepositRequest defines a deposit request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositRequest {
    /// id specifies the id for the request
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// pool_id specifies the pool id
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    /// msg_height specifies the block height when the request is stored for the batch execution
    #[prost(int64, tag = "3")]
    pub msg_height: i64,
    /// depositor specifies the bech32-encoded address that makes a deposit to the pool
    #[prost(string, tag = "4")]
    pub depositor: ::prost::alloc::string::String,
    /// deposit_coins specifies the amount of coins to deposit.
    #[prost(message, repeated, tag = "5")]
    pub deposit_coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// accepted_coins specifies the amount of coins that are accepted.
    #[prost(message, repeated, tag = "6")]
    pub accepted_coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub minted_pool_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(enumeration = "RequestStatus", tag = "8")]
    pub status: i32,
    #[prost(uint64, tag = "9")]
    pub app_id: u64,
}
/// WithdrawRequest defines a withdraw request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawRequest {
    /// id specifies the id for the request
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// pool_id specifies the pool id
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    /// msg_height specifies the block height when the request is stored for the batch execution
    #[prost(int64, tag = "3")]
    pub msg_height: i64,
    /// withdrawer specifies the bech32-encoded address that withdraws pool coin from the pool
    #[prost(string, tag = "4")]
    pub withdrawer: ::prost::alloc::string::String,
    /// pool_coin specifies the pool coin that is a proof of liquidity provider for the pool
    #[prost(message, optional, tag = "5")]
    pub pool_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// withdrawn_coins specifies the amount of coins that are withdrawn.
    #[prost(message, repeated, tag = "6")]
    pub withdrawn_coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(enumeration = "RequestStatus", tag = "7")]
    pub status: i32,
    #[prost(uint64, tag = "8")]
    pub app_id: u64,
}
/// Order defines an order.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// id specifies the id for the request
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// pair_id specifies the pair id
    #[prost(uint64, tag = "2")]
    pub pair_id: u64,
    /// msg_height specifies the block height when the request is stored for the batch execution
    #[prost(int64, tag = "3")]
    pub msg_height: i64,
    /// orderer specifies the bech32-encoded address that makes an order
    #[prost(string, tag = "4")]
    pub orderer: ::prost::alloc::string::String,
    /// direction specifies the order direction; either buy or sell
    #[prost(enumeration = "OrderDirection", tag = "5")]
    pub direction: i32,
    #[prost(message, optional, tag = "6")]
    pub offer_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// remaining_offer_coin specifies the remaining offer coin
    #[prost(message, optional, tag = "7")]
    pub remaining_offer_coin:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// received_coin specifies the received coin after the swap
    #[prost(message, optional, tag = "8")]
    pub received_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// price specifies the price that an orderer is willing to swap
    #[prost(string, tag = "9")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub open_amount: ::prost::alloc::string::String,
    /// batch_id specifies the pair's batch id when the request is stored
    #[prost(uint64, tag = "12")]
    pub batch_id: u64,
    #[prost(message, optional, tag = "13")]
    pub expire_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "OrderStatus", tag = "14")]
    pub status: i32,
    #[prost(uint64, tag = "15")]
    pub app_id: u64,
    /// type specifies the typo of the order
    #[prost(enumeration = "OrderType", tag = "16")]
    pub r#type: i32,
}
/// MMOrderIndex defines an index type to quickly find market making orders
/// from an orderer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MmOrderIndex {
    #[prost(string, tag = "1")]
    pub orderer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub pair_id: u64,
    #[prost(uint64, repeated, tag = "4")]
    pub order_ids: ::prost::alloc::vec::Vec<u64>,
}
// FARMING STRUCTURES - QUEUE AND ACTIVE

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveFarmer {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub farmer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub farmed_pool_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedCoin {
    #[prost(message, optional, tag = "1")]
    pub farmed_pool_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedFarmer {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub farmer: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub queud_coins: ::prost::alloc::vec::Vec<QueuedCoin>,
}
/// PoolType enumerates pool types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PoolType {
    /// POOL_TYPE_UNSPECIFIED specifies unknown pool type
    Unspecified = 0,
    /// POOL_TYPE_BASIC specifies the basic pool type
    Basic = 1,
    /// POOL_TYPE_RANGED specifies the ranged pool type
    Ranged = 2,
}
impl PoolType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PoolType::Unspecified => "POOL_TYPE_UNSPECIFIED",
            PoolType::Basic => "POOL_TYPE_BASIC",
            PoolType::Ranged => "POOL_TYPE_RANGED",
        }
    }
}
/// OrderType enumerates order types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    /// ORDER_TYPE_UNSPECIFIED specifies unknown order type.
    Unspecified = 0,
    /// ORDER_TYPE_LIMIT specifies limit order type.
    Limit = 1,
    /// ORDER_TYPE_MARKET specifies market order type.
    Market = 2,
    /// ORDER_TYPE_MM specifies MM(market making) order type.
    Mm = 3,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "ORDER_TYPE_UNSPECIFIED",
            OrderType::Limit => "ORDER_TYPE_LIMIT",
            OrderType::Market => "ORDER_TYPE_MARKET",
            OrderType::Mm => "ORDER_TYPE_MM",
        }
    }
}
/// OrderDirection enumerates order directions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderDirection {
    /// ORDER_DIRECTION_UNSPECIFIED specifies unknown order direction
    Unspecified = 0,
    /// ORDER_DIRECTION_BUY specifies buy(swap quote coin to base coin) order direction
    Buy = 1,
    /// ORDER_DIRECTION_SELL specifies sell(swap base coin to quote coin) order direction
    Sell = 2,
}
impl OrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderDirection::Unspecified => "ORDER_DIRECTION_UNSPECIFIED",
            OrderDirection::Buy => "ORDER_DIRECTION_BUY",
            OrderDirection::Sell => "ORDER_DIRECTION_SELL",
        }
    }
}
/// RequestStatus enumerates request statuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RequestStatus {
    /// REQUEST_STATUS_UNSPECIFIED specifies unknown request status
    Unspecified = 0,
    /// REQUEST_STATUS_NOT_EXECUTED indicates the request is not executed yet
    NotExecuted = 1,
    /// REQUEST_STATUS_SUCCEEDED indicates the request has been succeeded
    Succeeded = 2,
    /// REQUEST_STATUS_FAILED indicates the request is failed
    Failed = 3,
}
impl RequestStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RequestStatus::Unspecified => "REQUEST_STATUS_UNSPECIFIED",
            RequestStatus::NotExecuted => "REQUEST_STATUS_NOT_EXECUTED",
            RequestStatus::Succeeded => "REQUEST_STATUS_SUCCEEDED",
            RequestStatus::Failed => "REQUEST_STATUS_FAILED",
        }
    }
}
/// OrderStatus enumerates order statuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    /// ORDER_STATUS_UNSPECIFIED specifies unknown order status
    Unspecified = 0,
    /// ORDER_STATUS_NOT_EXECUTED indicates the order has not been executed yet
    NotExecuted = 1,
    /// ORDER_STATUS_NOT_MATCHED indicates the order has been executed but has no match
    NotMatched = 2,
    /// ORDER_STATUS_PARTIALLY_MATCHED indicates the order has been partially matched
    PartiallyMatched = 3,
    /// ORDER_STATUS_COMPLETED indicates the order has been fully matched and completed
    Completed = 4,
    /// ORDER_STATUS_CANCELED indicates the order has been canceled
    Canceled = 5,
    /// ORDER_STATUS_EXPIRED indicates the order has been expired
    Expired = 6,
}
impl OrderStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderStatus::Unspecified => "ORDER_STATUS_UNSPECIFIED",
            OrderStatus::NotExecuted => "ORDER_STATUS_NOT_EXECUTED",
            OrderStatus::NotMatched => "ORDER_STATUS_NOT_MATCHED",
            OrderStatus::PartiallyMatched => "ORDER_STATUS_PARTIALLY_MATCHED",
            OrderStatus::Completed => "ORDER_STATUS_COMPLETED",
            OrderStatus::Canceled => "ORDER_STATUS_CANCELED",
            OrderStatus::Expired => "ORDER_STATUS_EXPIRED",
        }
    }
}
/// AddressType enumerates the available types of a address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddressType {
    /// the 32 bytes length address type of ADR 028.
    AddressType32Bytes = 0,
    /// the default 20 bytes length address type.
    AddressType20Bytes = 1,
}
impl AddressType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AddressType::AddressType32Bytes => "ADDRESS_TYPE_32_BYTES",
            AddressType::AddressType20Bytes => "ADDRESS_TYPE_20_BYTES",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppGenesisState {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub generic_params: ::core::option::Option<GenericParams>,
    #[prost(uint64, tag = "3")]
    pub last_pair_id: u64,
    #[prost(uint64, tag = "4")]
    pub last_pool_id: u64,
    #[prost(message, repeated, tag = "5")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
    #[prost(message, repeated, tag = "6")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
    #[prost(message, repeated, tag = "7")]
    pub deposit_requests: ::prost::alloc::vec::Vec<DepositRequest>,
    #[prost(message, repeated, tag = "8")]
    pub withdraw_requests: ::prost::alloc::vec::Vec<WithdrawRequest>,
    #[prost(message, repeated, tag = "9")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, repeated, tag = "10")]
    pub active_farmers: ::prost::alloc::vec::Vec<ActiveFarmer>,
    #[prost(message, repeated, tag = "11")]
    pub queued_farmers: ::prost::alloc::vec::Vec<QueuedFarmer>,
    #[prost(message, repeated, tag = "12")]
    pub market_making_order_indexes: ::prost::alloc::vec::Vec<MmOrderIndex>,
}
/// GenesisState defines the liquidity module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub app_genesis_state: ::prost::alloc::vec::Vec<AppGenesisState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGenericParamsProposal {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNewLiquidityPairProposal {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(string, tag = "3")]
    pub base_coin_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub quote_coin_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryGenericParamsRequest is request type for the Query/GenericParams RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGenericParamsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
/// QueryGenericParamsResponse is response type for the Query/GenericParams RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGenericParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<GenericParams>,
}
/// QueryPoolsRequest is request type for the Query/Pools RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsRequest {
    #[prost(uint64, tag = "1")]
    pub pair_id: u64,
    #[prost(string, tag = "2")]
    pub disabled: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
}
/// QueryPoolsResponse is response type for the Query/Pools RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<PoolResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPoolRequest is request type for the Query/Pool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
}
/// QueryPoolResponse is response type for the Query/Pool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<PoolResponse>,
}
/// QueryPoolByReserveAddressRequest is request type for the Query/PoolByReserveAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolByReserveAddressRequest {
    #[prost(string, tag = "1")]
    pub reserve_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
}
/// QueryPoolByPoolCoinDenomRequest is request type for the Query/PoolByPoolCoinDenom RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolByPoolCoinDenomRequest {
    #[prost(string, tag = "1")]
    pub pool_coin_denom: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
}
/// QueryPairsRequest is request type for the Query/Pairs RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairsRequest {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(uint64, tag = "3")]
    pub app_id: u64,
}
/// QueryPairsResponse is response type for the Query/Pairs RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPairRequest is request type for the Query/Pair RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairRequest {
    #[prost(uint64, tag = "1")]
    pub pair_id: u64,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
}
/// QueryPairResponse is response type for the Query/Pair RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairResponse {
    #[prost(message, optional, tag = "1")]
    pub pair: ::core::option::Option<Pair>,
}
/// QueryDepositRequestsRequest is request type for the Query/DepositRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositRequestsRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDepositRequestsResponse is response type for the Query/DepositRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub deposit_requests: ::prost::alloc::vec::Vec<DepositRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryDepositRequestRequest is request type for the Query/DepositRequest RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositRequestRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(uint64, tag = "3")]
    pub app_id: u64,
}
/// QueryDepositRequestResponse is response type for the Query/DepositRequest RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositRequestResponse {
    #[prost(message, optional, tag = "1")]
    pub deposit_request: ::core::option::Option<DepositRequest>,
}
/// QueryWithdrawRequestsRequest is request type for the Query/WithdrawRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(uint64, tag = "3")]
    pub app_id: u64,
}
/// QueryWithdrawRequestsResponse is response type for the Query/WithdrawRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub withdraw_requests: ::prost::alloc::vec::Vec<WithdrawRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryWithdrawRequestRequest is request type for the Query/WithdrawRequest RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(uint64, tag = "3")]
    pub app_id: u64,
}
/// QueryWithdrawRequestResponse is response type for the Query/WithdrawRequest RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestResponse {
    #[prost(message, optional, tag = "1")]
    pub withdraw_request: ::core::option::Option<WithdrawRequest>,
}
/// QueryOrdersRequest is request type for the Query/Orders RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOrdersRequest {
    #[prost(uint64, tag = "1")]
    pub pair_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(uint64, tag = "3")]
    pub app_id: u64,
}
/// QueryOrdersResponse is response type for the Query/Orders RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryOrderRequest is request type for the Query/Order RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOrderRequest {
    #[prost(uint64, tag = "1")]
    pub pair_id: u64,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(uint64, tag = "3")]
    pub app_id: u64,
}
/// QueryOrderResponse is response type for the Query/Order RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
/// QueryOrdersByOrdererRequest is request type for the Query/OrdersByOrderer RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOrdersByOrdererRequest {
    #[prost(string, tag = "1")]
    pub orderer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pair_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
}
//
// Custom response messages
//

/// PoolResponse defines a custom pool response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub pair_id: u64,
    #[prost(string, tag = "3")]
    pub reserve_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pool_coin_denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub balances: ::core::option::Option<PoolBalances>,
    #[prost(uint64, tag = "6")]
    pub last_deposit_request_id: u64,
    #[prost(uint64, tag = "7")]
    pub last_withdraw_request_id: u64,
    #[prost(uint64, tag = "8")]
    pub app_id: u64,
    #[prost(enumeration = "PoolType", tag = "9")]
    pub r#type: i32,
    #[prost(string, tag = "10")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub pool_coin_supply: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub min_price: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub max_price: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub price: ::prost::alloc::string::String,
    #[prost(bool, tag = "15")]
    pub disabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolBalances {
    #[prost(message, optional, tag = "1")]
    pub base_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub quote_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryFarmerRequest is request type for the Query/Farmer RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFarmerRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub farmer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedPoolCoin {
    #[prost(message, optional, tag = "1")]
    pub pool_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub deque_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// QueryFarmerResponse is response type for the Query/Farmer RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFarmerResponse {
    #[prost(message, optional, tag = "1")]
    pub active_pool_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "2")]
    pub queued_pool_coin: ::prost::alloc::vec::Vec<QueuedPoolCoin>,
}
/// QueryDeserializePoolCoinRequest is request type for the Query/DeserializePoolCoin RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDeserializePoolCoinRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_coin_amount: u64,
    #[prost(uint64, tag = "3")]
    pub app_id: u64,
}
/// QueryDeserializePoolCoinResponse is response type for the Query/DeserializePoolCoin RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDeserializePoolCoinResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryPoolsIncentivesRequest is request type for the Query/PoolsIncentives RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsIncentivesRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolIncentive {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(bool, tag = "2")]
    pub master_pool: bool,
    #[prost(uint64, repeated, tag = "3")]
    pub child_pool_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "4")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub distributed_rewards:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "6")]
    pub total_epochs: u64,
    #[prost(uint64, tag = "7")]
    pub filled_epochs: u64,
    #[prost(message, optional, tag = "8")]
    pub epoch_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "9")]
    pub next_distribution: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "10")]
    pub is_swap_fee: bool,
    #[prost(uint64, tag = "11")]
    pub app_id: u64,
}
/// QueryPoolIncentivesResponse is response type for the Query/PoolsIncentives RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolIncentivesResponse {
    #[prost(message, repeated, tag = "1")]
    pub pool_incentives: ::prost::alloc::vec::Vec<PoolIncentive>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFarmedPoolCoinRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFarmedPoolCoinResponse {
    #[prost(message, optional, tag = "1")]
    pub coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryOrderBooksRequest is request type for the Query/OrderBooks RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOrderBooksRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, repeated, tag = "2")]
    pub pair_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, repeated, tag = "3")]
    pub price_unit_powers: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, tag = "4")]
    pub num_ticks: u32,
}
/// QueryOrderBooksResponse is response type for Query/OrderBooks RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOrderBooksResponse {
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<OrderBookPairResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookPairResponse {
    #[prost(uint64, tag = "1")]
    pub pair_id: u64,
    #[prost(string, tag = "2")]
    pub base_price: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub order_books: ::prost::alloc::vec::Vec<OrderBookResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookResponse {
    #[prost(string, tag = "1")]
    pub price_unit: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub sells: ::prost::alloc::vec::Vec<OrderBookTickResponse>,
    #[prost(message, repeated, tag = "3")]
    pub buys: ::prost::alloc::vec::Vec<OrderBookTickResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookTickResponse {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_order_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pool_order_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalActiveAndQueuedPoolCoins {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "2")]
    pub total_active_pool_coin:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub total_queued_pool_coin:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllFarmedPoolCoinsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllFarmedPoolCoinsResponse {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub total_active_and_queued_coins: ::prost::alloc::vec::Vec<TotalActiveAndQueuedPoolCoins>,
}
/// MsgCreatePair defines an SDK message for creating a pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePair {
    /// creator specifies the bech32-encoded address that is the pair creator.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// base_coin_denom specifies the base coin denom of the pair.
    #[prost(string, tag = "2")]
    pub base_coin_denom: ::prost::alloc::string::String,
    /// quote_coin_denom specifies the quote coin denom of the pair.
    #[prost(string, tag = "3")]
    pub quote_coin_denom: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePairResponse {}
/// MsgCreatePool defines an SDK message for creating a pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePool {
    /// creator specifies the bech32-encoded address that is the pool creator
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// pair_id specifies the pair id.
    #[prost(uint64, tag = "2")]
    pub pair_id: u64,
    /// deposit_coins specifies the amount of coins to deposit.
    #[prost(message, repeated, tag = "3")]
    pub deposit_coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
}
/// MsgCreatePoolResponse defines the Msg/CreatePool response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePoolResponse {}
/// MsgCreateRangedPool defines an SDK message for creating a ranged pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateRangedPool {
    /// creator specifies the bech32-encoded address that is the pool creator
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    /// pair_id specifies the pair id.
    #[prost(uint64, tag = "3")]
    pub pair_id: u64,
    /// deposit_coins specifies the amount of coins to deposit.
    #[prost(message, repeated, tag = "4")]
    pub deposit_coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub min_price: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub max_price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub initial_price: ::prost::alloc::string::String,
}
/// MsgCreateRangedPoolResponse defines the Msg/CreateRangedPool response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateRangedPoolResponse {}
/// MsgDeposit defines an SDK message for depositing coins to the pool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeposit {
    /// depositor specifies the bech32-encoded address that makes a deposit to the pool
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    /// pool_id specifies the pool id
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    /// deposit_coins specifies the amount of coins to deposit.
    #[prost(message, repeated, tag = "3")]
    pub deposit_coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
}
/// MsgDepositResponse defines the Msg/Deposit response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {}
/// MsgWithdraw defines an SDK message for withdrawing pool coin from the pool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdraw {
    /// withdrawer specifies the bech32-encoded address that withdraws pool coin from the pool
    #[prost(string, tag = "1")]
    pub withdrawer: ::prost::alloc::string::String,
    /// pool_id specifies the pool id
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    /// pool_coin specifies the pool coin that is a proof of liquidity provider for the pool
    #[prost(message, optional, tag = "3")]
    pub pool_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
}
/// MsgWithdrawResponse defines the Msg/Withdraw response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {}
/// MsgLimitOrder defines an SDK message for making a limit order
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLimitOrder {
    /// orderer specifies the bech32-encoded address that makes an order
    #[prost(string, tag = "1")]
    pub orderer: ::prost::alloc::string::String,
    /// pair_id specifies the pair id
    #[prost(uint64, tag = "2")]
    pub pair_id: u64,
    /// direction specifies the order direction(buy or sell)
    #[prost(enumeration = "OrderDirection", tag = "3")]
    pub direction: i32,
    /// offer_coin specifies the amount of coin the orderer offers
    #[prost(message, optional, tag = "4")]
    pub offer_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// demand_coin_denom specifies the demand coin denom
    #[prost(string, tag = "5")]
    pub demand_coin_denom: ::prost::alloc::string::String,
    /// price specifies the order price
    #[prost(string, tag = "6")]
    pub price: ::prost::alloc::string::String,
    /// amount specifies the amount of base coin the orderer wants to buy or sell
    #[prost(string, tag = "7")]
    pub amount: ::prost::alloc::string::String,
    /// order_lifespan specifies the order lifespan
    #[prost(message, optional, tag = "8")]
    pub order_lifespan: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, tag = "9")]
    pub app_id: u64,
}
/// MsgLimitOrderResponse defines the Msg/LimitOrder response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLimitOrderResponse {}
/// MsgMarketOrder defines an SDK message for making a market order
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMarketOrder {
    /// orderer specifies the bech32-encoded address that makes an order
    #[prost(string, tag = "1")]
    pub orderer: ::prost::alloc::string::String,
    /// pair_id specifies the pair id
    #[prost(uint64, tag = "2")]
    pub pair_id: u64,
    /// direction specifies the order direction(buy or sell)
    #[prost(enumeration = "OrderDirection", tag = "3")]
    pub direction: i32,
    /// offer_coin specifies the amount of coin the orderer offers
    #[prost(message, optional, tag = "4")]
    pub offer_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// demand_coin_denom specifies the demand coin denom
    #[prost(string, tag = "5")]
    pub demand_coin_denom: ::prost::alloc::string::String,
    /// amount specifies the amount of base coin the orderer wants to buy or sell
    #[prost(string, tag = "6")]
    pub amount: ::prost::alloc::string::String,
    /// order_lifespan specifies the order lifespan
    #[prost(message, optional, tag = "7")]
    pub order_lifespan: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, tag = "8")]
    pub app_id: u64,
}
/// MsgMarketOrderResponse defines the Msg/MarketOrder response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMarketOrderResponse {}
/// MsgMMOrder defines an SDK message for making a MM(market making) order.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMmOrder {
    /// orderer specifies the bech32-encoded address that makes an order
    #[prost(string, tag = "1")]
    pub orderer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    /// pair_id specifies the pair id
    #[prost(uint64, tag = "3")]
    pub pair_id: u64,
    /// max_sell_price specifies the maximum sell price
    #[prost(string, tag = "4")]
    pub max_sell_price: ::prost::alloc::string::String,
    /// min_sell_price specifies the minimum sell price
    #[prost(string, tag = "5")]
    pub min_sell_price: ::prost::alloc::string::String,
    /// sell_amount specifies the total amount of base coin of sell orders
    #[prost(string, tag = "6")]
    pub sell_amount: ::prost::alloc::string::String,
    /// max_buy_price specifies the maximum buy price
    #[prost(string, tag = "7")]
    pub max_buy_price: ::prost::alloc::string::String,
    /// min_buy_price specifies the minimum buy price
    #[prost(string, tag = "8")]
    pub min_buy_price: ::prost::alloc::string::String,
    /// buy_amount specifies the total amount of base coin of buy orders
    #[prost(string, tag = "9")]
    pub buy_amount: ::prost::alloc::string::String,
    /// order_lifespan specifies the order lifespan
    #[prost(message, optional, tag = "10")]
    pub order_lifespan: ::core::option::Option<::prost_types::Duration>,
}
/// MsgMMOrderResponse defines the Msg/MMOrder response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMmOrderResponse {}
/// MsgCancelOrder defines an SDK message for cancelling an order
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelOrder {
    /// orderer specifies the bech32-encoded address that makes an order
    #[prost(string, tag = "1")]
    pub orderer: ::prost::alloc::string::String,
    /// pair_id specifies the pair id
    #[prost(uint64, tag = "2")]
    pub pair_id: u64,
    /// order_id specifies the order id
    #[prost(uint64, tag = "3")]
    pub order_id: u64,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
}
/// MsgCancelOrderResponse defines the Msg/CancelOrder response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelOrderResponse {}
/// MsgCancelAllOrders defines an SDK message for cancelling all orders
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelAllOrders {
    /// orderer specifies the bech32-encoded address that makes an order
    #[prost(string, tag = "1")]
    pub orderer: ::prost::alloc::string::String,
    /// pair_ids specifies pair ids to cancel orders
    #[prost(uint64, repeated, tag = "2")]
    pub pair_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "3")]
    pub app_id: u64,
}
/// MsgCancelAllOrdersResponse defines the Msg/CancelAllOrders response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelAllOrdersResponse {}
/// MsgCancelMMOrder defines an SDK message for cancelling all market making orders
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelMmOrder {
    /// orderer specifies the bech32-encoded address that makes an order
    #[prost(string, tag = "1")]
    pub orderer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    /// pair_id specifies the pair id to cancel orders
    #[prost(uint64, tag = "3")]
    pub pair_id: u64,
}
/// MsgCancelMMOrderResponse defines the Msg/CancelMMOrder response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelMmOrderResponse {}
/// MsgFarm defines a SDK message for farming coins (i.e without bonding) for incentivisation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFarm {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub farmer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub farming_pool_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgFarmResponse  defines the Msg/MsgFarmResponse response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFarmResponse {}
/// MsgUnfarm defines a SDK message for performing unfarm of the farmed coins
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnfarm {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub farmer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub unfarming_pool_coin:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgUnfarmResponse defines the Msg/MsgUnfarmResponse response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnfarmResponse {}
include!("comdex.liquidity.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
