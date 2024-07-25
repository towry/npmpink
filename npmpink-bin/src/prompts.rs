use anyhow::Result;
use inquire::ui::{
    Attributes, Color as UiColor, ErrorMessageRenderConfig, RenderConfig, StyleSheet, Styled,
};
use inquire::{formatter::MultiOptionFormatter, MultiSelect};
use npmpink_core::package::Package;
use npmpink_tui::color::Color;

pub fn select_packages(pkgs: &[Package]) -> Result<(Vec<&Package>)> {
    let formatter: MultiOptionFormatter<'_, String> = &|a| format!("{} selected", a.len());
    let opts: Vec<String> = pkgs.iter().map(|p| p.dir.clone()).collect();
    let theme = create_theme();

    let ans = MultiSelect::new("Select packages:", opts)
        .with_render_config(theme)
        .with_formatter(formatter)
        .raw_prompt()?;

    Ok(ans
        .iter()
        .filter_map(|n| pkgs.get(n.index))
        .collect::<Vec<&Package>>())
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
