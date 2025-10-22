use module02_rust_track::config::Config;
use std::env;

#[test]
fn config_env_handling() {
    // Defaults
    env::remove_var("PORT");
    env::remove_var("APP_ENV");
    let c = Config::from_env();
    assert_eq!(c.port, 3000);
    assert_eq!(c.env, "dev");

    // From env
    env::set_var("PORT", "5055");
    env::set_var("APP_ENV", "test");
    let c2 = Config::from_env();
    assert_eq!(c2.port, 5055);
    assert_eq!(c2.env, "test");

    // Back to defaults
    env::remove_var("PORT");
    env::remove_var("APP_ENV");
    let c3 = Config::from_env();
    assert_eq!(c3.port, 3000);
    assert_eq!(c3.env, "dev");
}
