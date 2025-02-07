# DS210 B1 Spring 2025 Lectures

Lectures for DS210 Section B1 Spring 2025.

## Working with Mass Cloud

The Mass Cloud server environments are already configured for you.

### Jupyter Rust

Open a terminal and clone the repository.

### VSCode Rust

Select new terminal and clone the repository.


## Configuring Local Development Environment

Of course the first step is to clone this repo and the from a command shell
change into the root folder of this repository.

### Rust

To set up Rust, follow the directions on
[Getting started](https://www.rust-lang.org/learn/get-started).

### Latest Jupyter

```bash
# make sure you are in the root folder of this repo
python3 -m venv .venv
source .venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt
```

### Rust Notebook Kernel

To install Rust as a kernel option for Jupyter, follow these
[instructions](https://racum.blog/articles/rust-jupyter/).

### Rust Notebook Kernel on VSCode Rust Cloud

To run the Jupyter notebooks on the VSCode Rust cloud instance, do the following:

1. Install Jupyter vscode extension pack (from ms-toolsai)
2. install Python vscode extension (from ms-python)

3.  In vscode, open a terminal and run:

```sh
cargo install --locked evcxr_jupyter

evcxr_jupyter --install
```

4. Install rust-analyzer vscode extension (from rust-lang)

#### Select Rust Kernel for Jupyter Notebooks in VSCode

When you open a Jupyter notebook with Rust code cells, the Rust kernel may
not be selected. You might see a "Select Kernel" in the upper right of the
notebook.

To select the Rust kernel, click on "Select Kernel", then "Select another kernel...",
then "Jupyter kernel..." and then you should see an option to select "Rust" as
a kernel.

