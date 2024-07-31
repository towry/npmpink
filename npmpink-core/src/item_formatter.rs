use crate::item_display::PackageItemDisplay;
use crate::package::Package;
use crate::source::Source;
use std::rc::Weak;

struct SourceItemFormatter {}

pub struct PackageItemFormatter<'a> {
    pub inner: &'a Package,
    pub source: Weak<Source>,
}

impl<'a> PackageItemFormatter<'a> {
    pub fn new(package: &'a Package, source: Weak<Source>) -> PackageItemFormatter<'_> {
        PackageItemFormatter {
            inner: package,
            source,
        }
    }
}

impl<'a> From<PackageItemFormatter<'a>> for PackageItemDisplay {
    fn from(val: PackageItemFormatter<'a>) -> Self {
        PackageItemDisplay {
            title: val.inner.name.clone(),
            source_label: source_label(&val.source).unwrap_or("<unkown source>".to_owned()),
            source_id: source_id(&val.source).unwrap_or("<unkown source id>".to_owned()),
        }
    }
}

fn source_label(source: &Weak<Source>) -> Option<String> {
    let source = source.upgrade();
    let source = source?;

    source
        .path
        .to_owned()
        .file_stem()
        .and_then(|p| p.to_os_string().into_string().ok())
}

fn source_id(source: &Weak<Source>) -> Option<String> {
    let source = source.upgrade();
    let source = source?;

    Some(source.id.clone())
}
