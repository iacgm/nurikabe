pub mod board;
pub mod known;
pub mod rules;
pub mod solve;
pub mod ui;
pub mod utils;

pub use board::*;
pub use known::*;
pub use rules::*;
pub use solve::*;
pub use ui::*;
pub use utils::*;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
