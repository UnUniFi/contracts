use osmosis_std::types::osmosis::gamm::v1beta1::{MsgExitPool, MsgJoinPool, MsgSwapExactAmountIn};
use osmosis_std::types::osmosis::lockup::{MsgBeginUnlocking, MsgLockTokens};
use prost::EncodeError;
use prost_types::Any;


pub fn join_pool_to_any(msg: MsgJoinPool) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.gamm.v1beta1.MsgJoinPool".to_owned(),
        value: msg.to_proto_bytes(),
    });
}

pub fn exit_pool_to_any(msg: MsgExitPool) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.gamm.v1beta1.MsgExitPool".to_owned(),
        value: msg.to_proto_bytes(),
    });
}

pub fn swap_msg_to_any(msg: MsgSwapExactAmountIn) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn".to_owned(),
        value: msg.to_proto_bytes(),
    });
}

pub fn lock_tokens_msg_to_any(msg: MsgLockTokens) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.lockup.MsgLockTokens".to_owned(),
        value: msg.to_proto_bytes(),
    });
}

pub fn begin_unlocking_msg_to_any(msg: MsgBeginUnlocking) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.lockup.MsgBeginUnlocking".to_owned(),
        value: msg.to_proto_bytes(),
    });
}
