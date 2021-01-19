use std::env;

pub fn get_var(name: &str) -> String {
    env::var_os(name).unwrap().to_str().unwrap().to_string()
}