use const_format::{concatcp, str_replace};

/// Represent Steam's base api url.
pub const STEAMAPI_BASE: &'static str = "https://api.steampowered.com";

/// Represent Steam's community url.
pub const COMMUNITY_BASE: &'static str = "https://steamcommunity.com";

pub const MOBILEAUTH_BASE: &'static str = concatcp!(STEAMAPI_BASE, "/IMobileAuthService/%s/v0001");

/// Represent Steam's mobile authenticate token.
// FIXME: check document quality.
pub const MOBILEAUTH_GETWGTOKEN: &'static str = str_replace!(MOBILEAUTH_BASE, "%s", "GetWGToken");

const TWO_FACTOR_BASE: &'static str = concatcp!(STEAMAPI_BASE, "ITwoFactorService/%s/v0001");

/// Represent Steam's two-factor authenticate time query.
// FIXME: check document quality.
pub const TWO_FACTOR_TIME_QUERY: &'static str = str_replace!(TWO_FACTOR_BASE, "%s", "QueryTime");
