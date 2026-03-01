use bigdecimal::{
    BigDecimal, 
    num_bigint::BigInt,
    num_bigint::ToBigInt
};

pub fn parse_hex_i64(hex_string: &str) -> anyhow::Result<i64> {
    let hex_string = hex_string.trim_start_matches("0x");
    u64::from_str_radix(hex_string, 16)
        .map_err(|e| anyhow::anyhow!("Failed to parse hex string '{}': {}", e, hex_string))
        .map(|val| val as i64)
}

pub fn parse_hex(hex_string: &str) -> anyhow::Result<BigDecimal> {
    let hex_string = hex_string.trim_start_matches("0x");
    BigInt::parse_bytes(hex_string.as_bytes(), 16)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse hex string '{}'", hex_string))
        .map(|val| BigDecimal::from(val))
}

pub fn to_hex_string(value: &BigDecimal) -> String {
    let big_int = value.to_bigint().unwrap_or_else(|| BigInt::from(0));
    format!("0x{:x}", big_int)
}