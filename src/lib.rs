#[macro_use]
mod helpers;

use std::{
    fs::{self, File},
    io::{Error, Write},
    path::PathBuf,
};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    name: Option<String>,

    /// Config file for generator
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates new item in a layer
    New(NewArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct NewArgs {
    #[command(subcommand)]
    pub layer: Option<Layers>,
}

#[derive(Debug, Args, PartialEq, Eq, Hash)]
pub struct LayerArgs {
    name: Option<String>,
}

#[derive(Debug, Subcommand, Eq, PartialEq, Hash)]
pub enum Layers {
    Page(LayerArgs),
    Widget(LayerArgs),
    Feature(LayerArgs),
    Entity(LayerArgs),
    Shared(LayerArgs),
}

impl Layers {
    pub fn value(&self) -> String {
        match self {
            Layers::Page(_) => format!("Page"),
            Layers::Widget(_) => format!("Widget"),
            Layers::Feature(_) => format!("Feature"),
            Layers::Entity(_) => format!("Entity"),
            Layers::Shared(_) => format!("Shared"),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Layers::Page(arg)
            | Layers::Widget(arg)
            | Layers::Feature(arg)
            | Layers::Entity(arg)
            | Layers::Shared(arg) => {
                format!("{:?}", &arg.name.as_deref().unwrap())
            }
        }
    }

    pub fn get_path(&self) -> String {
        match self {
            Layers::Page(arg) => format!("./pages/{}", &arg.name.as_deref().unwrap()),
            Layers::Widget(arg) => format!("./widgets/{}", &arg.name.as_deref().unwrap()),
            Layers::Feature(arg) => format!("./features/{}", &arg.name.as_deref().unwrap()),
            Layers::Entity(arg) => format!("./entities/{}", &arg.name.as_deref().unwrap()),
            Layers::Shared(arg) => format!("./shared/{}", &arg.name.as_deref().unwrap()),
        }
    }

    pub fn get_segments(&self) -> Vec<String> {
        return vec_of_strings!("ui", "model", "lib", "api");
    }
}

pub fn create_layer_slice(layer: &Layers) {
    let path = layer.get_path();

    match fs::create_dir_all(&path) {
        Ok(_) => (),
        Err(err) => panic!("failed to create layer directory: {err}"),
    };

    create_layer_segments(layer, &path);

    let mut main_index_file = create_barrel_file(&path)
        .unwrap_or_else(|err| panic!("failed to create slice index.ts file: {err}"));

    for layer in layer.get_segments() {
        let export_line = format!("export {{}} from './{layer}' \n");

        main_index_file
            .write_all(export_line.as_bytes())
            .unwrap_or_else(|err| panic!("failed to write to index.ts file: {err}"))
    }

    println!(
        "Created new slice {} for layer {:?}",
        layer.name(),
        layer.value()
    )
}

fn create_layer_segments(layer: &Layers, path: &String) {
    let segments = layer.get_segments();

    for segment in segments {
        let segment_path = format!("{path}/{segment}");

        match fs::create_dir_all(&segment_path) {
            Ok(_) => (),
            Err(err) => panic!("failed to create segment directory: {err}"),
        };

        match create_barrel_file(&segment_path) {
            Ok(file) => file,
            Err(err) => panic!("failed to create segment index.ts file: {err}"),
        };
    }
}

fn create_barrel_file(path: &String) -> Result<File, Error> {
    return fs::File::create(format!("{}/index.ts", &path));
}
