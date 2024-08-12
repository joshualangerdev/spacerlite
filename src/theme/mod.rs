pub mod palette;
mod widgets;

pub mod prelude {
    pub use super::{
        //palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}
