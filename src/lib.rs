use std::env;
use std::collections::HashMap;

/// `EnvSwapper` temporarily sets environment variables and restores them after the scope ends.
pub struct EnvSwapper {
    original_env: HashMap<String, Option<String>>,
}

impl EnvSwapper {
    /// Creates a new `EnvSwapper` and sets the specified environment variables.
    pub fn new(vars: &[(&str, &str)]) -> Self {
        let mut original_env = HashMap::new();

        // Set each environment variable and store its original value
        for &(key, value) in vars {
            original_env.insert(key.to_string(), env::var(key).ok());
            env::set_var(key, value);
        }

        EnvSwapper { original_env }
    }
}

impl Drop for EnvSwapper {
    fn drop(&mut self) {
        // Restore each environment variable to its original value
        for (key, original_value) in &self.original_env {
            match original_value {
                Some(val) => env::set_var(key, val),
                None => env::remove_var(key),
            }
        }
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_temporarily_sets_and_reverts_env_vars() {
        let key = "TEMP_TEST_VAR";
        env::remove_var(key);  // Ensure variable is unset

        {
            let _swapper = EnvSwapper::new(&[(key, "temp_value")]);
            assert_eq!(env::var(key).unwrap(), "temp_value");
        }

        // Verify that the variable was reverted
        assert!(env::var(key).is_err());
    }

    #[test]
    fn it_reverts_to_original_value() {
        let key = "EXISTING_VAR";
        env::set_var(key, "original_value");

        {
            let _swapper = EnvSwapper::new(&[(key, "temp_value")]);
            assert_eq!(env::var(key).unwrap(), "temp_value");
        }

        // Confirm it reverts to the original value
        assert_eq!(env::var(key).unwrap(), "original_value");
    }
}
