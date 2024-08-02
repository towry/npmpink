use anstyle::{AnsiColor, Style};
use std::fmt;

use npmpink_core::{
    item_display::PackageItemDisplay as PackageItemDisplayInner,
    item_formatter::PackageItemFormatter,
};

#[derive(Clone)]
pub struct PackageItemDisplay<'a> {
    pub inner: PackageItemDisplayInner,
    pub raw: PackageItemFormatter<'a>,
}

impl<'a> PackageItemDisplay<'a> {
    pub fn new(formatter: PackageItemFormatter<'a>) -> PackageItemDisplay {
        PackageItemDisplay {
            inner: formatter.clone().into(),
            raw: formatter,
        }
    }
}

impl<'a> fmt::Display for PackageItemDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source_label_style = Style::new().fg_color(Some(AnsiColor::Blue.into())).bold();

        // TODO: apply style here
        write!(
            f,
            "{}  {source_label_style}{}{source_label_style:#}",
            self.inner.title, self.inner.source_label
        )
    }
}
