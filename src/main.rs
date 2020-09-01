use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
struct Cell {
    cell_type: String,
    metadata: Option<Value>,
    source: Vec<String>,
    execution_count: Option<u32>,
    outputs: Option<Vec<Value>>
}

#[derive(Serialize, Deserialize, Debug)]
struct Notebook {
    cells: Vec<Cell>,
    metadata: Value,
    nbformat: u32,
    nbformat_minor: u32
}

fn read_notebook_from_file<P: AsRef<Path>>(path: P) -> Result<Notebook, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let n: Notebook = serde_json::from_reader(reader)?;

    Ok(n)
}

fn filter(cell: &Cell) -> bool {
    !cell.source.contains(&String::from("<div style=\"outline: 2px dotted blue;\">\n"))
}

fn main() {
    let notebook: Notebook = read_notebook_from_file("input.ipynb").unwrap();

    let cells: Vec<Cell> = notebook.cells
        .into_iter()
        .filter(|cell| filter(cell))
        .collect();

    let result = Notebook {
        cells: cells,
        ..notebook
    };

    println!("{}", serde_json::to_string(&result).unwrap());
}
