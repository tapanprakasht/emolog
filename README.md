# [emolog](https://crates.io/crates/emolog)

emolog is a simple logging library which shows emojis in the log messages. 

[![](https://img.shields.io/crates/v/emolog.svg)](https://crates.io/crates/emolog) [![Docs](https://docs.rs/emolog/badge.svg)](https://docs.rs/emolog)

## Usage
```rust
use emolog::{emolog_critical, emolog_error, emolog_info, emolog_success, emolog_warn};

fn main() {
    emolog_info!("Your info message goes here");
    emolog_warn!("Your warning message goes here");
    emolog_success!("Your success message goes here");
    emolog_error!("Your error message goes here");
    emolog_critical!("Your critical message goes here");
}
``` 
Output

<img height="150px" src="./docs/output.png">