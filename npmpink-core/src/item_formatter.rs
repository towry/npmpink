use crate::item_display::PackageItemDisplay;
use crate::package::Package;
use crate::source::Source;
use std::rc::Rc;

struct SourceItemFormatter {}

#[derive(Clone)]
pub struct PackageItemFormatter<'a> {
    pub inner: Rc<Package>,
    pub source: &'a Source,
}

impl<'a> PackageItemFormatter<'a> {
    pub fn new(package: Rc<Package>, source: &'a Source) -> PackageItemFormatter<'a> {
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
            source_label: source_label(val.source).unwrap_or("<unkown source>".to_owned()),
            source_id: source_id(val.source).unwrap_or("<unkown source id>".to_owned()),
        }
    }
}

fn source_label(source: &Source) -> Option<String> {
    source
        .path
        .to_owned()
        .file_stem()
        .and_then(|p| p.to_os_string().into_string().ok())
}

fn source_id(source: &Source) -> Option<String> {
    Some(source.id.clone())
}
