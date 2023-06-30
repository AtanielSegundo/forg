# File Organizer by Type of File

This is a simple command-line tool written in Rust that organizes files in a directory based on their file types. It scans the specified directory and moves files into respective subdirectories based on their file extensions.

## Prerequisites

Before using this tool, ensure that you have the following installed:

- Rust programming language (https://www.rust-lang.org/tools/install)

## Usage

1. Clone the repository or download the source code.
2. Open a terminal or command prompt.
3. Navigate to the project directory.
4. Build the project using the following command:

   ```shell
   cargo build --release
   ```

5. Run the tool with the following command, specifying the directory to organize:

   ```shell
   ./target/release/file_organizer <directory_path>
   ```

   Replace `<directory_path>` with the path to the directory you want to organize.

6. The tool will scan the specified directory and organize the files based on their extensions into separate subdirectories.

## Code Overview

The code is structured as follows:

- The `Directory` struct holds two vectors: `file_names` for storing the names of files and `extensions` for storing their corresponding extensions.
- The `push` method of the `Directory` struct adds a tuple of `(name, extension)` to the respective vectors.
- The `FileNameParser` function takes a `std::ffi::OsString` representing a file name and splits it into the name and extension parts.
- The `main` function is the entry point of the program. It retrieves the input directory path from the command-line arguments, reads the directory contents, and organizes the files by calling `FileNameParser` and `push` methods of `Directory`.

Feel free to customize the `file_extensions` array in the `main` function to specify the file types you want to organize.

Please note that this tool will move the files within the specified directory, so use it with caution.

## License

This project is licensed under the [MIT License](LICENSE).
