# csvew (CSV View)

## Overview

`csvew` (CSV View) is a terminal user interface (TUI) application built with `ratatui` that allows users to view and interact with CSV files in a highly intuitive and user-friendly manner. This application provides a powerful way to inspect, filter, query, and order CSV data, all from within your terminal.

## Features

- **CSV Schema Display**: Displays the schema of the CSV file on one side of the interface, providing a clear view of the column names and their respective data types.
- **CSV Table View**: Shows the contents of the CSV file as a table, allowing for easy navigation and inspection of data.
- **Filter Block**: Allows users to filter the CSV data based on specific criteria.
- **Query Block**: Provides SQL-like query capabilities to perform complex data retrieval operations.
- **Order By Block**: Enables users to sort the CSV data based on specified columns.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/rhasanm/csvew.git
    cd csvew
    ```

2. **Build the Project**:
    Ensure you have Rust and Cargo installed. Then run:
    ```bash
    cargo build --release
    ```

3. **Run the Application**:
    ```bash
    cargo run --release -- <path_to_csv_file>
    ```

## Usage

### Command Line

To open a CSV file with `csvew`, use the following command:
```bash
csvew <path_to_csv_file>
```

### Keybindings

#### Normal Mode
- **`F`**: Enter Filter mode.
- **`Q`**: Enter Query mode.
- **`O`**: Enter Order By mode.
- **`T`**: Enter Table view mode.
- **`S`**: Enter Schema view mode.
- **`q`**: Quit the application.

#### Filter Mode
- **`Enter`**: Submit the filter.
- **Any character**: Add character to filter input.
- **`Backspace`**: Delete character from filter input.
- **`Left Arrow`**: Move cursor left in filter input.
- **`Right Arrow`**: Move cursor right in filter input.
- **`Esc`**: Exit Filter mode and return to Normal mode.

#### Schema Mode
- **`j` / `Down Arrow`**: Scroll down in the schema.
- **`k` / `Up Arrow`**: Scroll up in the schema.
- **`h` / `Left Arrow`**: Scroll left in the schema.
- **`l` / `Right Arrow`**: Scroll right in the schema.
- **`Esc`**: Exit Schema mode and return to Normal mode.

### Blocks
- **Tab**: Switch between different blocks (Schema, Table, Filter, Query, Order By).

## Contributing

We welcome contributions from the community! If you find a bug or have a feature request, please open an issue on the [GitHub repository](https://github.com/rhasanm/csvew). To contribute code, fork the repository, create a new branch with a descriptive name, make your changes, and open a pull request.

### Development Setup

1. **Fork the Repository**: Click the "Fork" button at the top of the repository page.
2. **Clone Your Fork**: 
    ```bash
    git clone https://github.com/yourusername/csvew.git
    cd csvew
    ```
3. **Create a Branch**:
    ```bash
    git checkout -b my-feature-branch
    ```
4. **Make Changes**: Implement your changes or new features.
5. **Run Tests**: Ensure all tests pass.
    ```bash
    cargo test
    ```
6. **Commit and Push**:
    ```bash
    git add .
    git commit -m "Description of your changes"
    git push origin my-feature-branch
    ```
7. **Open a Pull Request**: Go to the original repository and open a pull request from your fork.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Built with [ratatui](https://github.com/tui-rs-revival/ratatui), a Rust library for building terminal user interfaces.
- Inspired by the need for efficient CSV data inspection and manipulation in terminal environments.

---

Thank you for using `csvew`! If you have any questions or need further assistance, feel free to open an issue on GitHub or contact us at [your email address].
