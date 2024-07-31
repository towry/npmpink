/// |package_name|source_dir_tail|source_id|
#[derive(Clone)]
pub struct PackageItemDisplay {
    pub title: String,
    pub source_label: String,
    pub source_id: String,
}

#[derive(Clone)]
pub struct SourceItemDisplay {
    pub title: String,
}
