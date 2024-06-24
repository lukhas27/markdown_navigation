# Markdown Navigation

Creates a markdown navigation for all markdown files in passed folder.

**[Example](Example/Readme.md)**

## Usage
```

```

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
