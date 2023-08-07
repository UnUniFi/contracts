use crate::binding::AddressBytes;
use crate::error::ContractError;

/// Maximum length of address
pub const MAX_ADDR_LEN: usize = 255;

/// Decodes a bech32 encoded string and converts to base64 encoded bytes
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/types/bech32/bech32.go#L20
pub fn decode_and_convert(encoded: &str) -> Result<AddressBytes, ContractError> {
    let (_hrp, bytes, _variant) = bech32::decode(encoded)?;

    Ok(bech32::convert_bits(&bytes, 5, 8, false)?)
}

/// Prefixes the address bytes with its length
pub fn length_prefix<AddrBytes: AsRef<[u8]>>(addr: AddrBytes) -> Result<Vec<u8>, ContractError> {
    let bz_length = addr.as_ref().len();

    if bz_length == 0 {
        return Ok(vec![]);
    }

    if bz_length > MAX_ADDR_LEN {
        return Err(ContractError::MaxAddrLength {});
    }

    let mut p: Vec<u8> = vec![bz_length as u8];
    p.extend_from_slice(addr.as_ref());

    Ok(p)
}
