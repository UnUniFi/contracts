use crate::state::{DepositToken, ExternToken};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::IbcEndpoint;
use cosmwasm_std::{Coin, Uint128};
use strategy::v1::msgs::{EpochMsg, StakeMsg, UnstakeMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub unbond_period: u64,
    pub transfer_timeout: u64,
    pub controller_deposit_denom: String,
    pub base_denom: String,
    pub quote_denom: String,
    pub chain_id: String,
    pub pool_id: u64,
    pub lp_denom: String,
    pub transfer_channel_id: String,
    pub controller_transfer_channel_id: String,
    pub superfluid_validator: String,
    pub automate_superfluid: bool,
    pub extern_tokens: Vec<ExternToken>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    UpdateState(UpdateStateMsg),
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    SuperfluidDelegate(SuperfluidDelegateMsg),
    Epoch(EpochMsg),
    ResetUnbondRequestLpAmount(ResetUnbondRequestLpAmountMsg),
    ResetUnbondingsToBeginState(ResetUnbondingsToBeginStateMsg),
    ProcessInstantUnbondings(ProcessInstantUnbondingsMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
    pub deposit_token: Option<DepositToken>,
    pub unbond_period: Option<u64>,
    pub pool_id: Option<u64>,
    pub ica_channel_id: Option<String>,
    pub lp_denom: Option<String>,
    pub phase: Option<Phase>,
    pub phase_step: Option<PhaseStep>,
    pub transfer_timeout: Option<u64>,
    pub transfer_channel_id: Option<String>,
    pub quote_denom: Option<String>,
    pub base_denom: Option<String>,
    pub controller_deposit_denom: Option<String>,
    pub controller_transfer_channel_id: Option<String>,
    pub chain_id: Option<String>,
    pub superfluid_validator: Option<String>,
    pub automate_superfluid: Option<bool>,
    pub extern_tokens: Option<Vec<ExternToken>>,
}

#[cw_serde]
pub struct UpdateStateMsg {
    pub bonded_lp_amount: Option<Uint128>,
    pub unbonding_lp_amount: Option<Uint128>,
    pub total_shares: Option<Uint128>,
    pub total_deposit: Option<Uint128>,
    pub total_withdrawn: Option<Uint128>,
    pub pending_icq: Option<u64>,
    pub unbond_request_lp_amount: Option<Uint128>,
    pub free_lp_amount: Option<Uint128>,
    pub pending_bond_lp_amount: Option<Uint128>,
    pub pending_lp_removal_amount: Option<Uint128>,
    pub free_quote_amount: Option<Uint128>,
    pub free_base_amount: Option<Uint128>,
    pub controller_free_amount: Option<Uint128>,
    pub controller_pending_transfer_amount: Option<Uint128>,
    pub controller_stacked_amount_to_deposit: Option<Uint128>,
}

#[cw_serde]
pub struct ResetUnbondRequestLpAmountMsg {}

#[cw_serde]
pub struct ResetUnbondingsToBeginStateMsg {}

#[cw_serde]
pub struct InstantUnbonding {
    pub unbonding_id: u64,
    pub withdrawal: Uint128,
    pub share_recover_amount: Uint128,
    pub share_receiver: String,
}

#[cw_serde]
pub struct ProcessInstantUnbondingsMsg {
    pub unbondings: Vec<InstantUnbonding>,
}

#[cw_serde]
pub struct IcqBalanceCallbackMsg {
    pub coins: Vec<Coin>,
}

#[cw_serde]
pub struct IbcTransferCallbackMsg {}

#[cw_serde]
pub struct IbcTransferToHostMsg {}

#[cw_serde]
pub struct IbcTransferToControllerMsg {}

#[cw_serde]
pub struct IcaAddAndBondLiquidityMsg {}

#[cw_serde]
pub struct IcaRemoveLiquidityMsg {}

#[cw_serde]
pub struct IcaSwapRewardsToTwoTokensMsg {}

#[cw_serde]
pub struct IcaSwapTwoTokensToDepositTokenMsg {}

#[cw_serde]
pub struct IcaSwapBalanceToTwoTokensMsg {}

#[cw_serde]
pub struct IcaBondLpTokensMsg {}

#[cw_serde]
pub struct IcaBeginUnbondLpTokensMsg {
    pub unbonding_lp_amount: Uint128,
}

#[cw_serde]
pub struct SuperfluidDelegateMsg {}

#[cw_serde]
pub enum QueryMsg {
    Version {},
    DepositDenom {},
    Fee {},
    Amounts {
        addr: String,
    },
    Kyc {},
    Params {},
    State {},
    Bonded {
        addr: String,
    },
    Unbonding {
        addr: String,
    },
    /// Show all channels connected to.
    ListChannels {},
    /// Returns the details of the name channel, error if not created.
    Channel {
        id: String,
    },
    Unbondings {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ListChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[cw_serde]
pub struct ChannelResponse {
    /// Information on the channel's connection
    pub info: ChannelInfo,
}

#[cw_serde]
pub enum Phase {
    Deposit,
    Withdraw,
}

impl ToString for Phase {
    fn to_string(&self) -> String {
        match self {
            Phase::Deposit => String::from("deposit"),
            Phase::Withdraw => String::from("withdraw"),
        }
    }
}

#[cw_serde]
pub enum PhaseStep {
    // Deposit operations
    IbcTransferToHost,
    IbcTransferToHostCallback,
    RequestIcqAfterIbcTransferToHost,
    ResponseIcqAfterIbcTransferToHost,
    SellExternTokens,
    SellExternTokensCallback,
    RequestIcqAfterSellExternTokens,
    ResponseIcqAfterSellExternTokens,
    AddLiquidity,
    AddLiquidityCallback,
    BondLiquidity,
    BondLiquidityCallback,
    RequestIcqAfterBondLiquidity,
    ResponseIcqAfterBondLiquidity,
    BeginUnbondingForPendingRequests,
    BeginUnbondingForPendingRequestsCallback,
    CheckMaturedUnbondings,

    // Withdraw operations
    RemoveLiquidity,
    RemoveLiquidityCallback,
    RequestIcqAfterRemoveLiquidity,
    ResponseIcqAfterRemoveLiquidity,
    SwapTwoTokensToDepositToken,
    SwapTwoTokensToDepositTokenCallback,
    RequestIcqAfterSwapTwoTokensToDepositToken,
    ResponseIcqAfterSwapTwoTokensToDepositToken,
    IbcTransferToController,
    IbcTransferToControllerCallback,
    RequestIcqAfterIbcTransferToController,
    ResponseIcqAfterIbcTransferToController,
    DistributeToUnbondedUsers,
}

impl ToString for PhaseStep {
    fn to_string(&self) -> String {
        match self {
            PhaseStep::IbcTransferToHost => String::from("ibc_transfer_to_host"),
            PhaseStep::IbcTransferToHostCallback => String::from("ibc_transfer_to_host_callback"),
            PhaseStep::RequestIcqAfterIbcTransferToHost => {
                String::from("request_icq_after_ibc_transfer_to_host")
            }
            PhaseStep::ResponseIcqAfterIbcTransferToHost => {
                String::from("response_icq_after_ibc_transfer_to_host")
            }
            PhaseStep::SellExternTokens => String::from("sell_extern_tokens"),
            PhaseStep::SellExternTokensCallback => String::from("sell_extern_tokens_callback"),
            PhaseStep::RequestIcqAfterSellExternTokens => {
                String::from("request_icq_after_sell_extern_tokens")
            }
            PhaseStep::ResponseIcqAfterSellExternTokens => {
                String::from("response_icq_after_sell_extern_tokens")
            }
            PhaseStep::AddLiquidity => String::from("add_liquidity"),
            PhaseStep::AddLiquidityCallback => String::from("add_liquidity_callback"),
            PhaseStep::BondLiquidity => String::from("bond_liquidity"),
            PhaseStep::BondLiquidityCallback => String::from("bond_liquidity_callback"),
            PhaseStep::RequestIcqAfterBondLiquidity => {
                String::from("request_icq_after_bond_liquidity")
            }
            PhaseStep::ResponseIcqAfterBondLiquidity => {
                String::from("response_icq_after_bond_liquidity")
            }
            PhaseStep::BeginUnbondingForPendingRequests => {
                String::from("begin_unbonding_for_pending_requests")
            }
            PhaseStep::BeginUnbondingForPendingRequestsCallback => {
                String::from("begin_unbonding_for_pending_requests_callback")
            }
            PhaseStep::CheckMaturedUnbondings => String::from("check_matured_unbondings"),
            PhaseStep::RemoveLiquidity => String::from("remove_liquidity"),
            PhaseStep::RemoveLiquidityCallback => String::from("remove_liquidity_callback"),
            PhaseStep::RequestIcqAfterRemoveLiquidity => {
                String::from("request_icq_after_remove_liquidity")
            }
            PhaseStep::ResponseIcqAfterRemoveLiquidity => {
                String::from("response_icq_after_remove_liquidity")
            }
            PhaseStep::SwapTwoTokensToDepositToken => {
                String::from("swap_two_tokens_to_deposit_token")
            }
            PhaseStep::SwapTwoTokensToDepositTokenCallback => {
                String::from("swap_two_tokens_to_deposit_token_callback")
            }
            PhaseStep::RequestIcqAfterSwapTwoTokensToDepositToken => {
                String::from("request_icq_after_swap_two_tokens_to_deposit_token")
            }
            PhaseStep::ResponseIcqAfterSwapTwoTokensToDepositToken => {
                String::from("response_icq_after_swap_two_tokens_to_deposit_token")
            }
            PhaseStep::IbcTransferToController => String::from("ibc_transfer_to_controller"),
            PhaseStep::IbcTransferToControllerCallback => {
                String::from("ibc_transfer_to_controller_callback")
            }
            PhaseStep::RequestIcqAfterIbcTransferToController => {
                String::from("request_icq_after_ibc_transfer_to_controller")
            }
            PhaseStep::ResponseIcqAfterIbcTransferToController => {
                String::from("response_icq_after_ibc_transfer_to_controller")
            }
            PhaseStep::DistributeToUnbondedUsers => String::from("distribute_to_unbonded_users"),
        }
    }
}

pub fn phase_from_phase_step(step: PhaseStep) -> Phase {
    match step {
        PhaseStep::IbcTransferToHost => Phase::Deposit,
        PhaseStep::IbcTransferToHostCallback => Phase::Deposit,
        PhaseStep::RequestIcqAfterIbcTransferToHost => Phase::Deposit,
        PhaseStep::ResponseIcqAfterIbcTransferToHost => Phase::Deposit,
        PhaseStep::SellExternTokens => Phase::Deposit,
        PhaseStep::SellExternTokensCallback => Phase::Deposit,
        PhaseStep::RequestIcqAfterSellExternTokens => Phase::Deposit,
        PhaseStep::ResponseIcqAfterSellExternTokens => Phase::Deposit,
        PhaseStep::AddLiquidity => Phase::Deposit,
        PhaseStep::AddLiquidityCallback => Phase::Deposit,
        PhaseStep::BondLiquidity => Phase::Deposit,
        PhaseStep::BondLiquidityCallback => Phase::Deposit,
        PhaseStep::RequestIcqAfterBondLiquidity => Phase::Deposit,
        PhaseStep::ResponseIcqAfterBondLiquidity => Phase::Deposit,
        PhaseStep::BeginUnbondingForPendingRequests => Phase::Deposit,
        PhaseStep::BeginUnbondingForPendingRequestsCallback => Phase::Deposit,
        PhaseStep::CheckMaturedUnbondings => Phase::Deposit,
        _ => Phase::Withdraw,
    }
}

#[cw_serde]
pub struct ChannelInfo {
    /// id of this channel
    pub id: String,
    /// the remote channel/port we connect to
    pub counterparty_endpoint: IbcEndpoint,
    /// the connection this exists on (you can use to query client/consensus info)
    pub connection_id: String,
    /// interchain account address
    pub address: String,
}
