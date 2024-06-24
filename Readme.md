# Markdown Navigation

Creates a markdown navigation for all markdown files in passed folder.

**[Example](Example/Readme.md)**

## Usage

## Todo's

1. Read the markdown file.
2. Read all folders and file that are in the folder with the readme
   1. make link to `/folder/Readme.md` named "Folder"
   2. make link to `./file.md name` "File"
3. Check if there is a parent `Readme.md` or other file in folder available.
4. Create link to parent:
   1. **if folder**: `[<-](../Readme.md)
- [Example](<Example/Readme.md>)
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
