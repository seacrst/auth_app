pub struct AppConfig<'a> {
    pub paths: Paths<'a>
}

impl AppConfig<'_> {
    pub fn config() -> Self {
        let paths = Paths {
            login: "/login",
            logout: "/logout",
            signup: "/signup",
            verify_2fa: "/verify-2fa",
            verify_token: "/verify-token"
        };
        
        Self {paths}
    }
}

pub struct Paths<'a> {
    pub signup: &'a str,
    pub login: &'a str,
    pub logout: &'a str,
    pub verify_2fa: &'a str,
    pub verify_token: &'a str,
}