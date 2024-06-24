# Markdown Navigation

Creates a markdown navigation for all markdown files in passed folder.

**[Example](Example/Readme.md)**

## Installation

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) must be installed on your system.

### Steps

1. Clone the repository:

   ```sh
   git clone https://github.com/lukhas27/mdnav.git
   cd mdnav
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Move the binary to a directory in your PATH, for example `~/.local/bin/`:

   ```sh
   sudo mv target/release/mdnav ~/.local/bin/
   ```

4. Verify the installation:

   ```sh
   mdnav --version
   ```

## Easy Installation

To install the tool using the provided script, run:

```sh
curl -sSf https://raw.githubusercontent.com/lukhas27/mdnav/master/install.sh | sh
```
Enure `~/.local/bin/` is added to your path.

### Usage

Run following command:

```
mdnav <folder_path>
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Todo's

- [ ] Table of Content Integration

## What it does

1. Read the markdown file.
2. Read all folders and file that are in the folder with the readme
   1. make link to `/folder/Readme.md` named "folder"
   2. make link to `./file.md name` "file"
3. Check if there is a parent `Readme.md` or other files in folder available.
4. Create link to parent:
   1. **if folder**: `[<-](../Readme.md)`
   2. **if file and not** `Readme.md`: `[<-](/Readme.md)`
5. Look if the links already exist in markdown
   1. if not place them after the `[<-](../Readme.md)`
   2. if yes do nothing
6. Place them in alphabetic order
7. to update it watch out that there are no duplicates.

The `Readme.md` file in the following folder:

```
├── Page 1/
├── Page 2/
├── Readme.md
├── example 1.md
```

should look like this at beginning of file:

```
- [example 1](<example 1.md>)
- [Page 1](<Page 1/Readme.md>)
- [Page 2](<Page 2/Readme.md>)
---
```

or with parent `Readme.md`:

```
[<-](../Readme.md)
- [example 1](<example 1.md>)
- [Page 1](<Page 1/Readme.md>)
- [Page 2](<Page 2/Readme.md>)
---
```
