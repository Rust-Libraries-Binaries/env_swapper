# EnvSwapper

`env_swapper` is a lightweight Rust library to temporarily set environment variables, automatically restoring the previous state when out of scope.

## Installation

Add `env_swapper` to your `Cargo.toml`:

```toml
[dependencies]
env_swapper = "0.1.0"```

## Usage

use env_swapper::EnvSwapper;
use std::env;

fn main() {
    let key = "EXAMPLE_VAR";
    
    {
        let _swapper = EnvSwapper::new(&[(key, "temporary_value")]);
        println!("Inside scope: {:?}", env::var(key)); // Outputs "temporary_value"
    }

    println!("Outside scope: {:?}", env::var(key)); // Restored to original
}

## Features
Temporarily sets and restores environment variables automatically on scope exit.
Ideal for testing, sandboxed configurations, and controlled environment changes.

## License
This project is licensed under the MIT License.

