use std::collections::BTreeMap;
use std::fs::{read_to_string, File};
use std::io::Write;

use convert_case::{Case, Casing};
use serde::Deserialize;

type Icons = BTreeMap<String, Icon>;

#[derive(Clone, Debug, Deserialize)]
struct Icon {
    changes: Vec<String>,
    ligatures: Vec<String>,
    search: serde_json::Value,
    styles: Vec<String>,
    unicode: String,
    label: String,
    // voted: bool,
    svg: serde_json::Value,
    free: Vec<String>,
}

fn shorten_style(style: &str) -> Option<(&str, &str)> {
    match style {
        "brands" => Some(("fab", "Brand")),
        "solid" => Some(("fas", "Solid")),
        "regular" => Some(("far", "Regular")),
        _ => None,
    }
}

fn write_style(writer: &mut impl Write, style: &str, name: &str, multiple: bool) {
    if let Some((style, suffix)) = shorten_style(style) {
        let append = multiple.then(|| suffix).unwrap_or_default();
        writer.write_fmt(format_args!("\t#[fmt(\"{} fa-{}\")]\n", style, name));
        writer.write_fmt(format_args!("\t{}{},\n", name.to_case(Case::Pascal), append));
    }
}

fn export_icons(writer: &mut impl Write, icons: BTreeMap<String, Icon>) {
    writer.write(b"#[derive(Debug, Clone, Copy, Display)]\n");
    writer.write(b"pub enum Icons {\n");

    for (name, icon) in icons {
        let multiple = icon.styles.len() != 1;
        for style in icon.styles {
            write_style(writer, &style, &name, multiple)
        }
    }

    writer.write(b"}\n");
}

async fn import_from_github() -> Result<Icons, Box<dyn std::error::Error>> {
    let url =
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/master/metadata/icons.json";
    Ok(reqwest::get(url).await?.json().await?)
}

fn import_from_file() -> Result<Icons, Box<dyn std::error::Error>> {
    let path = "data.json";
    Ok(serde_json::from_str(&read_to_string(path)?)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let icons = import_from_file()?;

    // export_icons(&mut std::io::stdout(), icons);
    export_icons(&mut File::create("enum.txt")?, icons);

    Ok(())
}
