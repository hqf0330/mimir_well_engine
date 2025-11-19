//! 
//! 
//! A Rust library generated from the hqf0330/template template.

/// A simple function that returns a greeting message.
/// 
/// # Examples
/// 
/// ```
/// use mimir_well_engine::greet;
/// 
/// assert_eq!(greet("World"), "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }
}
