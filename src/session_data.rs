use reqwest::cookie;

pub struct SessionData<'a> {
    pub session_id: &'a str,
    pub steam_login: &'a str,
    pub steam_login_secure: &'a str,
    pub web_cookie: &'a str,
    pub o_auth_token: &'a str,
    pub steam_id: u64
}

impl<'a> SessionData<'a> {
    pub fn add_cookies(&self, cookies: cookie::Jar) {
        let domain = &".steamcommunity.com".parse::<reqwest::Url>().unwrap();

        cookies.add_cookie_str("mobileClientVersion=0%20(2.1.3); Path=/", domain);
        cookies.add_cookie_str("mobileClient=android; Path=/", domain);

        cookies.add_cookie_str(&("steamid=".to_owned() + &self.steam_id.to_string() + "; Path=/"), domain);
        cookies.add_cookie_str(&("steamLogin=".to_owned() + self.steam_login + "; Path=/; HttpOnly"), domain);

        cookies.add_cookie_str(&("steamLoginSecure=".to_owned() + self.steam_login_secure + "; Path=/; Secure; HttpOnly"), domain);
        cookies.add_cookie_str("Steam_Language=english; Path=/", domain);
        cookies.add_cookie_str("dob=; Path=/", domain);
        cookies.add_cookie_str(&("sessionid=".to_owned() + self.session_id + " Path=/"), domain);
    }
}
