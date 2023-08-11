use crate::state::DepositToken;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::IbcEndpoint;
use cosmwasm_std::{Coin, Decimal, Uint128};

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
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    ExecuteEpoch(ExecuteEpochMsg),
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub owner: Option<String>,
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
}

#[cw_serde]
pub struct IcqBalanceCallbackMsg {
    pub coins: Vec<Coin>,
}

#[cw_serde]
pub struct IbcTransferCallbackMsg {}

#[cw_serde]
pub struct StakeMsg {}

#[cw_serde]
pub struct UnstakeMsg {
    pub amount: Uint128,
}

#[cw_serde]
pub struct ExecuteEpochMsg {}

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
pub enum QueryMsg {
    Config {},
    State {},
    Bonded {
        addr: String,
    },
    Unbonding {
        addr: String,
    },
    Fee {},
    /// Show all channels connected to.
    ListChannels {},
    /// Returns the details of the name channel, error if not created.
    Channel {
        id: String,
    },
    Unbondings {},
}

#[cw_serde]
pub struct FeeInfo {
    pub deposit_fee_rate: Decimal,
    pub withdraw_fee_rate: Decimal,
    pub interest_fee_rate: Decimal,
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
            Phase::Deposit => String::from("Deposit"),
            Phase::Withdraw => String::from("Withdraw"),
        }
    }
}

#[cw_serde]
pub enum PhaseStep {
    // Deposit operations
    IbcTransferToHost,
    IbcTransferToHostCallback,
    RequestICQAfterIbcTransferToHost,
    ResponseICQAfterIbcTransferToHost,
    AddLiquidity,
    AddLiquidityCallback,
    BondLiquidity,
    BondLiquidityCallback,
    RequestICQAfterBondLiquidity,
    ResponseICQAfterBondLiquidity,
    BeginUnbondingForPendingRequests,
    BeginUnbondingForPendingRequestsCallback,
    CheckMaturedUnbondings,

    // Withdraw operations
    RemoveLiquidity,
    RemoveLiquidityCallback,
    RequestICQAfterRemoveLiquidity,
    ResponseICQAfterRemoveLiquidity,
    SwapTwoTokensToDepositToken,
    SwapTwoTokensToDepositTokenCallback,
    RequestICQAfterSwapTwoTokensToDepositToken,
    ResponseICQAfterSwapTwoTokensToDepositToken,
    IbcTransferToController,
    IbcTransferToControllerCallback,
    RequestICQAfterIbcTransferToController,
    ResponseICQAfterIbcTransferToController,
    DistributeToUnbondedUsers,
}

impl ToString for PhaseStep {
    fn to_string(&self) -> String {
        match self {
            PhaseStep::IbcTransferToHost => String::from("IbcTransferToHost"),
            PhaseStep::IbcTransferToHostCallback => String::from("IbcTransferToHostCallback"),
            PhaseStep::RequestICQAfterIbcTransferToHost => {
                String::from("RequestICQAfterIbcTransferToHost")
            }
            PhaseStep::ResponseICQAfterIbcTransferToHost => {
                String::from("ResponseICQAfterIbcTransferToHost")
            }
            PhaseStep::AddLiquidity => String::from("AddLiquidity"),
            PhaseStep::AddLiquidityCallback => String::from("AddLiquidityCallback"),
            PhaseStep::BondLiquidity => String::from("BondLiquidity"),
            PhaseStep::BondLiquidityCallback => String::from("BondLiquidityCallback"),
            PhaseStep::RequestICQAfterBondLiquidity => String::from("RequestICQAfterBondLiquidity"),
            PhaseStep::ResponseICQAfterBondLiquidity => {
                String::from("ResponseICQAfterBondLiquidity")
            }
            PhaseStep::BeginUnbondingForPendingRequests => {
                String::from("BeginUnbondingForPendingRequests")
            }
            PhaseStep::BeginUnbondingForPendingRequestsCallback => {
                String::from("BeginUnbondingForPendingRequestsCallback")
            }
            PhaseStep::CheckMaturedUnbondings => String::from("CheckMaturedUnbondings"),
            PhaseStep::RemoveLiquidity => String::from("RemoveLiquidity"),
            PhaseStep::RemoveLiquidityCallback => String::from("RemoveLiquidityCallback"),
            PhaseStep::RequestICQAfterRemoveLiquidity => {
                String::from("RequestICQAfterRemoveLiquidity")
            }
            PhaseStep::ResponseICQAfterRemoveLiquidity => {
                String::from("ResponseICQAfterRemoveLiquidity")
            }
            PhaseStep::SwapTwoTokensToDepositToken => String::from("SwapTwoTokensToDepositToken"),
            PhaseStep::SwapTwoTokensToDepositTokenCallback => {
                String::from("SwapTwoTokensToDepositTokenCallback")
            }
            PhaseStep::RequestICQAfterSwapTwoTokensToDepositToken => {
                String::from("RequestICQAfterSwapTwoTokensToDepositToken")
            }
            PhaseStep::ResponseICQAfterSwapTwoTokensToDepositToken => {
                String::from("ResponseICQAfterSwapTwoTokensToDepositToken")
            }
            PhaseStep::IbcTransferToController => String::from("IbcTransferToController"),
            PhaseStep::IbcTransferToControllerCallback => {
                String::from("IbcTransferToControllerCallback")
            }
            PhaseStep::RequestICQAfterIbcTransferToController => {
                String::from("RequestICQAfterIbcTransferToController")
            }
            PhaseStep::ResponseICQAfterIbcTransferToController => {
                String::from("ResponseICQAfterIbcTransferToController")
            }
            PhaseStep::DistributeToUnbondedUsers => String::from("DistributeToUnbondedUsers"),
        }
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
