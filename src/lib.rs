
#[macro_export]
macro_rules! emolog_warn {
    ($str:expr) => {
        println!("⚠️  {}", $str);
    };
}

#[macro_export]
macro_rules! emolog_critical {
    ($str:expr) => {
        println!("🔥 {}", $str);
    };
}

#[macro_export]
macro_rules! emolog_info {
    ($str:expr) => {
        println!("ℹ️  {}", $str);
    };
}

#[macro_export]
macro_rules! emolog_success{
    ($str:expr) => {
        println!("✅ {}", $str);
    };
}

#[macro_export]
macro_rules! emolog_error {
    ($str:expr) => {
        println!("❌ {}", $str);
    };
}