use crate::msg::{AllOriginalPacketMsg, ExecuteMsg,};
use cosmwasm_std::{from_slice, Binary, IbcMsg};


pub fn decode_packet(data: Binary) -> Result<AllOriginalPacketMsg, Box<dyn std::error::Error>> {
    let packet_type: AllOriginalPacketMsg = from_slice(&data)?;

    match packet_type {
        AllOriginalPacketMsg::OriginalMsg(_) => {
            let deserialized_data: ExecuteMsg = from_slice(&data)?;
            Ok(AllOriginalPacketMsg::OriginalMsg(deserialized_data))
        },
        AllOriginalPacketMsg::IbcMsg(_) => {
            let deserialized_data: IbcMsg = from_slice(&data)?;
            Ok(AllOriginalPacketMsg::IbcMsg(deserialized_data))
        },
        _ => Err("Unknown packet type".into()),
    }
}
