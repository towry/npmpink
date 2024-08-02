use anyhow::Result;
use std::fmt::Display;
use std::io::Write;
use std::marker::PhantomData;
use std::process::{Command, Stdio};
use std::thread;

use super::Picker;

#[derive(Default)]
pub struct FzfPickerConfig {}

pub struct FzfPicker<T> {
    config: FzfPickerConfig,
    _marker: PhantomData<T>,
}

impl<T> FzfPicker<T> {
    pub fn new(config: Option<FzfPickerConfig>) -> FzfPicker<T> {
        FzfPicker {
            _marker: PhantomData,
            config: config.unwrap_or_default(),
        }
    }
}

impl<T: Display + Clone> Picker for FzfPicker<T> {
    type Item = T;

    fn select(&self, input_items: &[Self::Item]) -> Result<Vec<Self::Item>> {
        let mut fzf_child = Command::new("fzf")
            .args([
                "--multi",
                "--bind",
                r"tab:select+down,shift-tab:deselect+up",
            ])
            .args(["--with-nth", "2.."])
            .args(["--ansi"])
            .args(["--header", "tab:select,s-tab:deselect,enter:accept"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut feed_item_fd = fzf_child.stdin.take().expect("Failed to open fzf stdin");

        let items = input_items
            .iter()
            .enumerate()
            .map(|(i, x)| format!("{} {}", i, x))
            .collect::<Vec<String>>();

        thread::spawn(move || {
            for item in items.iter() {
                let _ = writeln!(feed_item_fd, "{}", item);
            }
        });

        let output = fzf_child.wait_with_output().expect("Failed to read stdout");

        let select_index = parse_selected_index(
            String::from_utf8_lossy(&output.stdout)
                .trim_end()
                .to_owned(),
        );

        let select_items = select_index
            .iter()
            .map(|i| input_items[*i].clone())
            .collect::<Vec<Self::Item>>();

        Ok(select_items)
    }

    fn format_item(&self, item: &Self::Item) -> Option<Box<dyn Display>> {
        Some(Box::new(item.to_string()))
    }
}

fn parse_selected_index(input: String) -> Vec<usize> {
    input
        .lines()
        .map(|e| {
            // split with whitespace and the first is index number
            e.split_whitespace().next().unwrap().parse().unwrap()
        })
        .collect()
}
