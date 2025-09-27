# Project Context for poly_cls

## Project Overview
This is a Rust-based project called `poly_cls` that appears to be designed for processing CSV files, likely for customs or tender data classification. The project is configured to work with various LLM (Large Language Model) APIs including Ollama, OpenAI, DeepSeek, and Anthropic. It also includes configuration for PostgreSQL database connectivity and has a modular architecture with configuration management, CSV processing capabilities, and schema definitions.

## Project Details
- **Name:** poly_cls
- **Version:** 0.1.0
- **Language:** Rust
- **Edition:** 2024
- **Purpose:** CSV data processing and classification using LLMs with PostgreSQL integration

## Dependencies
The project uses several key dependencies:
- `serde` - For serialization/deserialization
- `tokio-postgres` - For PostgreSQL connectivity
- `tracing` - For logging and tracing
- `reqwest` - For HTTP requests with JSON support
- `csv` - For CSV file processing
- `config` - For configuration management (from the conf.rs file)

## File Structure
- **src/** - Contains the main source code with modules:
  - `main.rs` - Entry point of the application (currently has a basic "Hello, world!" implementation)
  - `imports.rs` - Contains file processing utilities and CSV handling
  - `conf.rs` - Configuration management with support for multiple LLM protocols and PostgreSQL
  - `schemas.rs` - Schema definitions (currently has a placeholder struct)
- **samples/** - Contains sample CSV files for customs data:
  - `eas_customs_1.csv` - East Asia customs data
  - `kz_customs_1.csv` - Kazakhstan customs data
  - `rus_customs_1.csv` - Russia customs data
  - `report_all.csv` - Combined report data

## LLM Support
The project is designed to work with multiple LLM providers:
- Ollama
- OpenAI
- DeepSeek
- Anthropic

Configuration for these is managed through config files that can define multiple LLM instances with different parameters.

## Database Integration
The project includes PostgreSQL integration with configuration support for:
- Host connection details
- User credentials
- Password management

## Building and Running
To build and run the project:
```bash
# Build the project
cargo build

# Run the project
cargo run
```

## Configuration
The application uses configuration files in INI format for:
- Main application settings
- Tender plan API configuration
- PostgreSQL settings
- Multiple LLM configurations (up to 254 different LLMs can be configured)

## Current State
The project appears to be in early development stages, with the main function currently showing a placeholder "Hello, world!" implementation. The core architecture for CSV processing, configuration management, and LLM integration is established but implementation details are still being developed.

## Development Conventions
- Code formatting follows rustfmt rules with 2-space tabs and 70-character width limit
- Configuration is managed via external files
- Structured logging with tracing
- Modular design with separate files for different concerns