// mod.rs
// 📦 Module entry point for GA components.
// Exports GA logic and submodules.

pub mod ga;
pub mod individual;
pub mod selection;
pub mod crossover;
pub mod mutation;
pub mod utils; 

pub use ga::run_ga;
pub use ga::run_ga_config;
pub use ga::run_ga_parallel;
