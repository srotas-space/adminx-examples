// src/config/env_vars.rs
use dotenv::dotenv;
use std::env;
use once_cell::sync::OnceCell;

static ENVIRONMENT: OnceCell<String> = OnceCell::new();
static INIT: OnceCell<()> = OnceCell::new();

pub fn load_environment() {
    INIT.get_or_init(|| {
        // Load .env file first
        dotenv().ok();
        
        // Then get the environment variable (which might now be set by .env)
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        
        // Set our static environment
        ENVIRONMENT.set(environment.clone()).ok();
        
        println!("Environment loaded: {}", environment);
    });
}


pub fn get_env() -> String {
    ENVIRONMENT.get().cloned().unwrap_or_else(|| "development".to_string())
}

pub fn get_custom_env(env_var: &str, default: &str) -> String {
    env::var(env_var).unwrap_or_else(|_| default.to_string())
}

pub fn is_production() -> bool {
    get_env() == "production"
}

pub fn is_development() -> bool {
    get_env() == "development"
}

pub fn is_staging() -> bool {
    get_env() == "staging"
}
