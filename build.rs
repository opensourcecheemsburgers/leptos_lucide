#![feature(let_chains)]

use core::panic;
use std::{
	fs::ReadDir,
	path::{Path, PathBuf},
	rc::Rc,
};

use anyhow::{Error, Result};

pub static ICONS_DIR: &'static str = "src/icons/";
pub static ICONS_MOD_FILE: &'static str = "src/icons/mod.rs";

#[derive(Clone)]
struct LucideSvgFile {
	pascal_name: Rc<str>,
	snake_name: Rc<str>,
	contents: Rc<str>,
}

impl LucideSvgFile {
	fn new(pascal_name: Rc<str>, snake_name: Rc<str>, contents: Rc<str>) -> Self {
		Self { pascal_name, snake_name, contents }
	}
}

#[derive(Clone)]
struct LucideComponent {
	mod_entry: Rc<str>,
	component_file_contents: Rc<str>,
	component_file_path: Rc<str>,
}

impl LucideComponent {
	fn new(
		mod_entry: Rc<str>,
		component_file_contents: Rc<str>,
		component_file_path: Rc<str>,
	) -> Self {
		Self { mod_entry, component_file_contents, component_file_path }
	}
}

fn main() -> Result<(), Error> {
	match Path::new("src/icons/dog.rs").exists() {
		true => Ok(()),
		false => {
			let files = fetch_lucide_svg_files()?;
			let components = create_lucide_components(files)?;
			write_lucide_mod_file(&components)?;
			write_lucide_component_files(&components)?;
			Ok(())
		}
	}
}

fn fetch_lucide_svg_files() -> Result<Vec<LucideSvgFile>, Error> {
	let paths = fetch_lucide_svg_paths()?;
	let svg_files = create_lucide_file_structs(paths)?;
	Ok(svg_files)
}

fn fetch_lucide_svg_paths() -> Result<Vec<PathBuf>, Error> {
	let read_dir = Path::new(ICONS_DIR).read_dir()?;
	let mut paths = Vec::new();
	for dir_entry in read_dir {
		if let Ok(dir_entry) = dir_entry
			&& dir_entry.file_name().to_string_lossy().ends_with(".svg")
		{
			paths.push(dir_entry.path())
		}
	}
	Ok(paths)
}

fn create_lucide_file_structs(paths: Vec<PathBuf>) -> Result<Vec<LucideSvgFile>, Error> {
	let files = paths
		.into_iter()
		.map(|path| lucide_svg_file_from_path(&path).unwrap())
		.collect::<Vec<LucideSvgFile>>();
	Ok(files)
}

fn lucide_svg_file_from_path(path: &PathBuf) -> Result<LucideSvgFile, Error> {
	let contents = std::fs::read_to_string(&path)?;
	let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
	let pascal_name = to_pascal_case(file_name.clone());
	let snake_name = to_snake_case(file_name);

	let lucide_file =
		LucideSvgFile::new(Rc::from(pascal_name), Rc::from(snake_name), Rc::from(contents));
	Ok(lucide_file)
}

fn to_pascal_case(mut file_name: String) -> String {
	file_name = file_name.replace(".svg", "");

	let words = file_name.split('-').collect::<Vec<&str>>();

	let mut pascal_file_name = String::new();
	words.iter().for_each(|word| {
		pascal_file_name.push_str(&&uppercase_first(word));
	});

	pascal_file_name
}

fn to_snake_case(mut file_name: String) -> String {
	file_name.pop();
	file_name.pop();
	file_name.pop();
	file_name.pop();

	file_name = file_name.replace("-", "_");
	file_name
}

fn uppercase_first(data: &str) -> String {
	let mut result = String::new();
	let mut first = true;
	for value in data.chars() {
		if first {
			result.push(value.to_ascii_uppercase());
			first = false;
		} else {
			result.push(value);
		}
	}
	result
}

fn create_lucide_components(
	lucide_files: Vec<LucideSvgFile>,
) -> Result<Vec<LucideComponent>, Error> {
	let lucide_components = lucide_files
		.into_iter()
		.map(|lucide_file| lucide_file_to_component(lucide_file).unwrap())
		.collect::<Vec<LucideComponent>>();
	Ok(lucide_components)
}

fn lucide_file_to_component(lucide_file: LucideSvgFile) -> Result<LucideComponent, Error> {
	let mod_entry = create_mod_entry(&lucide_file);
	let component_file_path = create_component_file_path(&lucide_file);
	let component_file_contents = create_component_file(&lucide_file);

	let lucide_component = LucideComponent::new(
		Rc::from(mod_entry),
		Rc::from(component_file_contents),
		Rc::from(component_file_path),
	);
	Ok(lucide_component)
}

fn create_mod_entry(lucide_file: &LucideSvgFile) -> String {
	let mut entry = "mod ".to_string();

	let snake_name = match lucide_file.snake_name.eq_ignore_ascii_case("box")
		|| lucide_file.snake_name.eq_ignore_ascii_case("move")
		|| lucide_file.snake_name.eq_ignore_ascii_case("type")
	{
		true => {
			let mut snake_name = "r#".to_string();
			snake_name.push_str(&lucide_file.snake_name);
			Rc::from(snake_name)
		}
		false => lucide_file.snake_name.clone(),
	};

	entry.push_str(&snake_name);
	entry.push_str(";\r\npub use ");
	entry.push_str(&snake_name);
	entry.push_str("::");
	entry.push_str(&lucide_file.pascal_name);
	entry.push_str(";\r\n\r\n");

	entry
}

fn create_component_file_path(lucide_file: &LucideSvgFile) -> String {
	let mut snake_name = lucide_file.snake_name.to_string();
	snake_name.insert_str(0, "src/icons/");
	snake_name.push_str(".rs");
	snake_name
}

fn create_component_file(lucide_file: &LucideSvgFile) -> String {
	let mut contents = lucide_file.contents.to_string();

	kill_parent(&mut contents);
	create_leptos_component(contents, lucide_file.pascal_name.clone())
}

fn kill_parent(contents: &mut String) {
	contents.replace_range(0..=200, "");
	contents.pop();
	contents.pop();
	contents.pop();
	contents.pop();
	contents.pop();
	contents.pop();
	contents.pop();
}

fn create_leptos_component(svg: String, pascal_name: Rc<str>) -> String {
	let mut icon_template = TEMPLATE.to_string();
	icon_template.remove(69); // ( ͡° ͜ʖ ͡°)
	icon_template.insert_str(69, &pascal_name); // ( ͡° ͜ʖ ͡°)
	icon_template = icon_template.replace("{}", &svg);
	icon_template
}

fn write_lucide_mod_file(lucide_components: &Vec<LucideComponent>) -> Result<(), Error> {
	let mod_file = lucide_components
		.iter()
		.map(|component| component.mod_entry.to_string())
		.collect::<Vec<String>>()
		.concat();
	std::fs::write(ICONS_MOD_FILE, mod_file)?;
	Ok(())
}

fn write_lucide_component_files(lucide_components: &Vec<LucideComponent>) -> Result<(), Error> {
	lucide_components.iter().for_each(|component| {
		std::fs::write(
			component.component_file_path.to_string(),
			component.component_file_contents.to_string(),
		)
		.expect("Couldn't write lucide component file.");
	});
	Ok(())
}

pub static TEMPLATE: &'static str = r#"use leptos::*;

use crate::LucideAttributesCtx;

#[component]
pub fn _() -> impl IntoView {
	let attributes = move || expect_context::<LucideAttributesCtx>().0.get();

    view! {
        <svg
            class=attributes().classes()
            xmlns=attributes().xmlns()
            width=attributes().width()
            height=attributes().height()
            viewBox=attributes().view_box()
            fill=attributes().fill()
            stroke=attributes().stroke()
            stroke-width=attributes().stroke_width()
            stroke-linecap=attributes().stroke_linecap()
            stroke-linejoin=attributes().stroke_linejoin()
        >
			{}
		</svg>
    }
}"#;
