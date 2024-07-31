#[path = "inquire.rs"]
mod inquire_picker;

use anyhow::Result;
pub use inquire_picker::{InquirePicker, InquirePickerConfig};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

trait Picker {
    type Item: Display;

    fn select(&self, items: &[Self::Item]) -> Result<Vec<Self::Item>>;

    fn format_item(&self, item: &Self::Item) -> Option<Box<dyn Display>>;
}

/// The list pickers that this crate supports.
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PickerMode {
    #[default]
    Inquire,
    Fzf,
}

#[derive(Default)]
pub struct PickConfig {
    pub mode: PickerMode,
    // pub fzf: Option<FzfPickerConfig>,
    pub inquire: Option<InquirePickerConfig>,
}

pub fn pick_items<I: Display + Clone>(items: &[I], config: Option<PickConfig>) -> Result<Vec<I>> {
    let config = config.unwrap_or_default();
    let picker: Box<dyn Picker<Item = I>> = match config.mode {
        PickerMode::Inquire => Box::new(InquirePicker::new(config.inquire)),
        _ => panic!("unknown picker mode"),
    };

    picker.select(items)
}
