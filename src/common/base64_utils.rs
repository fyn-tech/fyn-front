/* ------------------------------------------------------------------------------------------------
 * Fyn-Front: Modern CFD/CAE Web Interface
 * Copyright (C) 2025 Fyn-Front Authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 * ------------------------------------------------------------------------------------------------
 * filename: token.rs
 * description: Token utilities for decoding base64url-encoded token payloads
 * ------------------------------------------------------------------------------------------------
 */

use base64::Engine;
use serde::de::DeserializeOwned;

pub fn decode_base64_json<T>(input: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let input_standard = input.replace('-', "+").replace('_', "/");
    let padding = match input_standard.len() % 4 {
        2 => "==",
        3 => "=",
        _ => "",
    };
    let input_padded = format!("{}{}", input_standard, padding);

    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input_padded.as_bytes())
        .map_err(|e| format!("Failed to decode base64: {:?}", e))?;

    serde_json::from_slice(&decoded)
        .map_err(|e| format!("Failed to parse JSON: {:?}", e))
}
