use serde::Serialize;

use crate::validate::validate_no_secret_values_in_json;

pub fn print_json<T: Serialize>(value: &T) -> anyhow::Result<()> {
    let value = serde_json::to_value(value)?;
    validate_no_secret_values_in_json(&value).map_err(anyhow::Error::msg)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
