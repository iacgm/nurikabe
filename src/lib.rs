pub mod board;
pub mod rules;
pub mod solve;
pub mod ui;
pub mod utils;
pub mod annotation;

pub use board::*;
pub use rules::*;
pub use solve::*;
pub use ui::*;
pub use utils::*;
pub use annotation::*;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
