# notebook-tools
Rust CLI tools for manipulation of Jupyter Notebooks.

## Usage

```bash
$ notebook-tools --help

USAGE:
    notebook-tools [FLAGS] [OPTIONS] <input>

ARGS:
    <input>    Sets the input file

FLAGS:
    -c, --clear_code      Clear code cell source
    -o, --clear_output    Clear code cell output
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -f, --filter <filter_cells>    Filter cells by source

$ notebook-tools notebook.ipynb --filter background-color --clear_code --clear_output > notebook-processed.ipynb 
```
