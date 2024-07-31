use std::fmt;

use npmpink_core::{
    item_display::PackageItemDisplay as PackageItemDisplayInner,
    item_formatter::PackageItemFormatter,
};

#[derive(Clone)]
pub struct PackageItemDisplay(pub PackageItemDisplayInner);

impl PackageItemDisplay {
    pub fn new(formatter: PackageItemFormatter) -> PackageItemDisplay {
        PackageItemDisplay(formatter.into())
    }
}

impl fmt::Display for PackageItemDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: apply style here
        write!(f, "{}{}", self.0.title, self.0.source_label)
    }
}
