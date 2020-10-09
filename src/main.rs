use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::clap_app;

#[derive(Serialize, Deserialize, Debug)]
struct Cell {
    cell_type: CellType,
    metadata: Option<Value>,
    source: Vec<String>,
    execution_count: Option<u32>,
    outputs: Option<Vec<Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum CellType {
    code,
    markdown,
    raw,
}

#[derive(Serialize, Deserialize, Debug)]
struct Notebook {
    cells: Vec<Cell>,
    metadata: Value,
    nbformat: u32,
    nbformat_minor: u32,
}

fn read_notebook_from_file<P: AsRef<Path>>(path: P) -> Result<Notebook, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let n: Notebook = serde_json::from_reader(reader)?;

    Ok(n)
}

fn tag_in_source(source: &Vec<String>, tag: &String) -> bool {
    source.iter().any(|line| line.contains(tag))
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Ben Scholtz")
        (about: "Jupyter Notebook tools")
        (@arg input: +required "Sets the input file")
        (@arg filter_cells: -f --filter +takes_value "Filter cells by source")
        (@arg clear_code: -c --clear_code "Clear code cell source")
        (@arg clear_output: -o --clear_output "Clear code cell output")
    )
    .get_matches();

    let notebook_source = matches.value_of("input").unwrap();
    let notebook: Notebook = read_notebook_from_file(notebook_source).unwrap();

    let cells: Vec<Cell> = notebook
        .cells
        .into_iter()
        .filter(|cell| {
            if matches.is_present("filter_cells") {
                let tag: String = matches.value_of("filter_cells").unwrap().into();
                // &String::from("background-color")
                match cell.cell_type {
                    CellType::code => !tag_in_source(&cell.source, &tag),
                    CellType::markdown => !tag_in_source(&cell.source, &tag),
                    CellType::raw => !tag_in_source(&cell.source, &tag),
                }
            } else {
                true
            }
        })
        .map(|cell| {
            if matches.is_present("clear_code") {
                match cell.cell_type {
                    CellType::code => Cell {
                        source: Vec::new(),
                        ..cell
                    },
                    CellType::markdown => cell,
                    CellType::raw => cell,
                }
            } else {
                cell
            }
        })
        .map(|cell| {
            if matches.is_present("clear_output") {
                match cell.cell_type {
                    CellType::code => Cell {
                        outputs: Some(Vec::new()),
                        ..cell
                    },
                    CellType::markdown => cell,
                    CellType::raw => cell,
                }
            } else {
                cell
            }
        })
        .collect();

    let result = Notebook {
        cells: cells,
        ..notebook
    };

    println!("{}", serde_json::to_string(&result).unwrap());
}
