# repo-to-gpt

`repo-to-gpt` is a CLI tool designed to scan a repository or folder, collect its source code, and export it as JSON. Its intended use is to allow users to upload this JSON to a GPT chat session to get more direct responses about their code and suggestions for improvement.

**Caution**: This tool collects and exports your entire repository or a specific folder. It is your responsibility to use it wisely and securely. Be cautious about uploading sensitive or proprietary information.

**Disclaimer**: You should not use this utility on repositories you do not have permission to analyze. Unless you are a contributor or decision-maker for a repository, avoid using this tool on its contents.

## Features

- Scans directories recursively.
- Supports `.gitignore` for excluding files and directories.
- Filters files by extension.
- Outputs JSON in minified or pretty-printed format.
- Provides detailed logs on processed and skipped files.

## Installation

To install the tool, clone the repository and build it using `cargo`:

```bash
git clone github.com:mbround18/repo-to-gpt.git
cd repo-to-gpt
cargo install --path .
```

## Usage

### Basic Command

Run the tool with the following basic command:

```bash
repo-to-gpt --input <INPUT_FOLDER> --output <OUTPUT_FILE>
```

### Flags and Options

- `--input <INPUT_FOLDER>` (required): Specifies the folder to scan.
- `--output <OUTPUT_FILE>` (optional, default: `gpt.json`): Specifies the output JSON file.
- `--excluded <EXCLUDED_DIRS>`: Comma-separated list of directories to exclude (e.g., `node_modules,dist`).
- `--ignored <IGNORED_EXTENSIONS>`: Comma-separated list of file extensions to ignore (e.g., `.log,.tmp`).
- `--use-gitignore`: Use `.gitignore` to exclude files and directories.
- `--pretty`: Output JSON in a pretty-printed format instead of minified.

### Example Commands

#### Scan a Folder and Output Minified JSON

```bash
repo-to-gpt --input ./my_project --output my_project.json
```

#### Use `.gitignore` and Pretty-Print JSON

```bash
repo-to-gpt --input ./my_project --output my_project.json --use-gitignore --pretty
```

#### Customize Excluded Directories and Ignored Extensions

```bash
repo-to-gpt --input ./my_project --output my_project.json --excluded dist,tmp --ignored .log,.bak
```

## Logs and Feedback

The tool provides detailed logs during execution:

- Displays the number of files processed and skipped.
- Highlights errors encountered during file reading.

Example log output:

```
Scanning folder: ./my_project
Ignoring file extensions: [".log", ".tmp"]
Processed 42 files, skipped 10 files.
Folder contents successfully exported to minified JSON: my_project.json
```

## Contribution

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the BSD 3-Clause License. See the LICENSE file for details.


