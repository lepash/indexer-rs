pub fn parse_hex(hex_string: &str) -> anyhow::Result<i64> {
    let hex_string = hex_string.trim_start_matches("0x");
    u64::from_str_radix(hex_string, 16)
        .map_err(|e| anyhow::anyhow!("Failed to parse hex string '{}': {}", hex_string, e))
        .map(|val| val as i64)
}