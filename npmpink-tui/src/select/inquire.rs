use super::Picker;
use crate::color::Color;
use anyhow::Result;
use inquire::ui::{
    Attributes, Color as UiColor, ErrorMessageRenderConfig, RenderConfig, StyleSheet, Styled,
};
use inquire::{formatter::MultiOptionFormatter, MultiSelect};
use std::fmt::Display;
use std::marker::PhantomData;

struct InquirePicker<T> {
    _marker: PhantomData<T>,
}

impl<T: Display + Clone> Picker for InquirePicker<T> {
    type Item = T;

    fn select(&self, items: &[Self::Item]) -> Result<Vec<Self::Item>> {
        let formatter: MultiOptionFormatter<'_, String> = &|a| format!("{} selected", a.len());
        let opts: Vec<String> = items
            .iter()
            .map(|p| {
                self.format_item(p)
                    .map(|p| p.to_string())
                    .unwrap_or("".to_string())
            })
            .collect();
        let theme = create_theme();

        let ans = MultiSelect::new("Select packages:", opts)
            .with_render_config(theme)
            .with_formatter(formatter)
            .raw_prompt()?;

        Ok(ans
            .into_iter()
            .filter_map(|n| items.get(n.index))
            .cloned()
            .collect::<Vec<T>>())
    }

    fn format_item(&self, item: &Self::Item) -> Option<Box<dyn Display>> {
        Some(Box::new(item.to_string()))
    }
}

fn rgb(color: Color) -> UiColor {
    UiColor::AnsiValue(color as u8)
}

fn create_theme() -> RenderConfig<'static> {
    RenderConfig::empty()
        .with_default_value(StyleSheet::new().with_fg(rgb(Color::Pink)))
        .with_answer(
            StyleSheet::new()
                .with_fg(rgb(Color::Purple))
                .with_attr(Attributes::BOLD),
        )
        // Prefixes
        .with_prompt_prefix(Styled::new("›").with_fg(rgb(Color::Blue)))
        .with_answered_prompt_prefix(Styled::new("✔").with_fg(rgb(Color::Green)))
        .with_scroll_up_prefix(Styled::new("▴").with_fg(rgb(Color::GrayLight)))
        .with_scroll_down_prefix(Styled::new("▾").with_fg(rgb(Color::GrayLight)))
        .with_highlighted_option_prefix(Styled::new("›").with_fg(rgb(Color::Teal)))
        // States
        .with_help_message(StyleSheet::new().with_fg(rgb(Color::Purple)))
        .with_error_message(
            ErrorMessageRenderConfig::empty()
                .with_prefix(Styled::new("✘").with_fg(rgb(Color::Red)))
                .with_message(StyleSheet::new().with_fg(rgb(Color::Red))),
        )
        .with_canceled_prompt_indicator(Styled::new("(skipped)").with_fg(rgb(Color::Gray)))
        // Selects
        .with_selected_option(Some(StyleSheet::new().with_fg(rgb(Color::Teal))))
        .with_selected_checkbox(Styled::new("◉").with_fg(rgb(Color::Teal)))
        .with_unselected_checkbox(Styled::new("◯").with_fg(rgb(Color::GrayLight)))
}
