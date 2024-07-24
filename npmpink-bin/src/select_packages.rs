use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect,
};

use npmpink_core::package::Package;

pub fn select_packages(pkgs: &[Package]) {
    let formatter: MultiOptionFormatter<'_, String> = &|a| format!("{} different fruits", a.len());
    let opts: Vec<String> = pkgs.iter().map(|p| p.dir.clone()).collect();

    let ans = MultiSelect::new("Select the fruits for your shopping list:", opts)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(list) => println!("I'll get right on it"),
        Err(_) => println!("The shopping list could not be processed"),
    }
}
