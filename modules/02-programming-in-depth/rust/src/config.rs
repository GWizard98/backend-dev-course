use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub env: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { port: 3000, env: "dev".into() }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Some(p) = env::var("PORT").ok().and_then(|v| v.parse::<u16>().ok()) {
            cfg.port = p;
        }
        if let Ok(e) = env::var("APP_ENV") {
            if !e.is_empty() { cfg.env = e; }
        }
        cfg
    }
}
