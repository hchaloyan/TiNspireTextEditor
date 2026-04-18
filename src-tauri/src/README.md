# TiNspireTextEditor

This project is a text editor designed for creating and exporting TI-Nspire documents. It utilizes the Tauri framework to provide a lightweight desktop application that can generate `.tns` files from user input.

## Project Structure

The project is organized into several modules, each responsible for different functionalities:

- **src-tauri/src/main.rs**: Entry point of the application. Initializes the Tauri application, sets up plugins, and registers command handlers.
- **src-tauri/src/commands/**: Contains command-related functionalities.
  - **export.rs**: Implements the export functionality to generate TNS files.
  - **calculator.rs**: Contains stub implementations for interacting with TI-Nspire calculators.
- **src-tauri/src/luna/**: Handles interactions with the Luna binary.
  - **resolver.rs**: Resolves the path to the Luna binary.
- **src-tauri/src/xml/**: Manages XML generation for the TNS files.
  - **problem.rs**: Builds the XML structure for the problem.
- **src-tauri/src/models/**: Defines data models used in the application.
  - **export_result.rs**: Represents the result of the export operation.
- **src-tauri/src/utils/**: Contains utility functions.
  - **filename.rs**: Provides functions for sanitizing filenames.
- **src-tauri/src/tests/**: Contains unit tests for various functionalities.

## Usage Guidelines

- Use the application to create and edit text for TI-Nspire documents.
- Export your work as a `.tns` file, which can be transferred to a TI-Nspire calculator.
- Ensure that the Luna binary is correctly placed in the expected directory for exporting to work.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.