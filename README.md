# Taxi Data Management and Report Generation

My parents have a taxi company and had a need for a program to create monthly reports. :D

I initially made a spreadsheet for them but thought that I could use this chance to create a
program that has real life use. Also this is a good opportunity to play around with cross-compiling
and deploying the program to a Windows platform from my macbook.

## Dependencies

- `eframe`: For creating the GUI.
- `egui_extras`: For additional GUI components like the date picker.
- `chrono`: For date and time handling.
- `rusqlite`: For SQLite database operations.
- `printpdf`: For generating PDF reports.

## Installation

1. Ensure you have Rust installed. If not, install Rust from [rust-lang.org](https://www.rust-lang.org/).
2. Clone this repository:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```
3. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

1. Run the program:
    ```sh
    cargo run --release
    ```
2. Use the GUI to enter data:
    - Select a date using the date picker.
    - Enter the kilometers driven and income details.
    - Click "Tallenna" to save the data.
    - Click "Luo Raportti" to generate a PDF report for the selected month.

## Database

The program uses a SQLite database to store the data. The database file is located at `data/data.db`.
The database is created and initialized automatically if it doesn't exist.

## PDF Report

Work in progress.. 🚧

## Project Structure

- `src/main.rs`: Entry point of the program.
- `src/ui.rs`: Contains the code for building the GUI.
- `src/db.rs`: Handles database operations.
- `src/pdf.rs`: Contains the code for generating PDF reports.
- `data/`: Directory for the SQLite database file.

## License

This project is licensed under the MIT License.