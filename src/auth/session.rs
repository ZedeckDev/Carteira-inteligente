use crate::models::UserSession;
use tower_cookies::{Cookie, Cookies};

pub const SESSION_COOKIE_NAME: &str = "ci_session";

pub fn set_session_cookie(cookies: &Cookies, user_id: &str, name: &str, email: &str) {
    let session = UserSession {
        user_id: user_id.to_string(),
        name: name.to_string(),
        email: email.to_string(),
    };

    if let Ok(serialized) = serde_json::to_string(&session) {
        let mut cookie = Cookie::new(SESSION_COOKIE_NAME, serialized);
        cookie.set_path("/");
        cookie.set_http_only(true);
        cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
        // Expiração de 7 dias
        cookie.set_max_age(tower_cookies::cookie::time::Duration::days(7));
        cookies.add(cookie);
    }
}

pub fn clear_session_cookie(cookies: &Cookies) {
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, "");
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_max_age(tower_cookies::cookie::time::Duration::seconds(0));
    cookies.remove(cookie);
}

pub fn get_session(cookies: &Cookies) -> Option<UserSession> {
    let cookie = cookies.get(SESSION_COOKIE_NAME)?;
    serde_json::from_str::<UserSession>(cookie.value()).ok()
}
