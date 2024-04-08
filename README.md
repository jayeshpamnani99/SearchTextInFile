# SearchTextInFile

This is a simple command-line tool written in Rust that allows you to search for a given query within a specified file. It reads the file contents, searches for the query, and prints out the matching lines.

## Features

- Accepts command-line arguments for the search query and file path
- Reads the file contents and searches for the given query
- Prints out the matching lines
- Provides error handling for various scenarios, such as missing arguments or file read errors

## Usage

To use the SearchTextInFile Service, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/jayeshpamnani99/SearchTextInFile
   ```

2. Build the project:

   ```
   cargo build 
   ```

3. Run the executable with the search query and file path as arguments:

   ```
   cargo run -- "your_query_to_be_searched" "path/to/file.txt"
   ```

   Replace `"your_query"` with the text you want to search for, and `"path/to/file.txt"` with the path to the file you want to search.

## Example

Suppose you have a file named `example.txt` with the following content:

```
The quick brown fox jumps over the lazy dog.
The rain in Spain falls mainly on the plain.
Hello, world!
```

To search for the word "the" in this file, you would run the following command:

```
cargo run -- the path/to/example.txt
```

This would output:

```
The quick brown fox jumps over the lazy dog.
The rain in Spain falls mainly on the plain.
```

## Error Handling

The SearchTextInFile Service handles various errors that can occur during execution:

- If the user does not provide enough arguments, the program will print an error message and exit with a non-zero status code.
- If there is an error reading the file, the program will print an error message and exit with a non-zero status code.
