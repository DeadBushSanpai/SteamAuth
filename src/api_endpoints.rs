/// Represent Steam®'s base api url.
pub const STEAMAPI_BASE: &str = "https://api.steampowered.com";

/// Represent Steam®'s community url.
pub const COMMUNITY_BASE: &str = "https://steamcommunity.com";

const MOBILEAUTH_BASE: &str = const_format::concatcp!(STEAMAPI_BASE, "/IMobileAuthService/%s/v0001");

/// Represent Steam®'s mobile authenticate token.
// FIXME: check document quality.
pub static MOBILEAUTH_GETWGTOKEN: &str = const_format::str_replace!(MOBILEAUTH_BASE, "%s", "GetWGToken");

const TWO_FACTOR_BASE: &str = const_format::concatcp!(STEAMAPI_BASE, "ITwoFactorService/%s/v0001");

/// Represent Steam®'s two-factor authenticate time query.
// FIXME: check document quality.
pub static TWO_FACTOR_TIME_QUERY: &str = const_format::str_replace!(TWO_FACTOR_BASE, "%s", "QueryTime");
