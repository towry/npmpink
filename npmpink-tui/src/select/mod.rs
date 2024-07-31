pub mod inquire;

use anyhow::Result;
use std::fmt::Display;

pub trait Picker {
    type Item: Display;

    fn select(&self, items: &[Self::Item]) -> Result<Vec<Self::Item>>;

    fn format_item(&self, item: &Self::Item) -> Option<Box<dyn Display>>;
}
