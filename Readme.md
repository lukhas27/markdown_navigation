# Markdown Navigation

1. Read the markdown file.
2. Read all folders and file that are in the folder with the readme
   1. make link to `/folder/Readme.md` named "Folder"
   2. make link to `./file.md name` "File"
3. Check if there is a parent `Readme.md` or other file in folder available.
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
├── Basic Config.md
├── Docker.md
├── Minimal Website Boot screen/
│   ├── Readme.md
│  ──
├── Readme.md
├── Webserver.md
```

should look like this at beginning of file :

```
[<-](../Readme.md)

- [Example](<Example/Readme.md>)
---
```
