[<-](../Readme.md)

- [example 1](<example 1.md>)
- [Page 1](<Page 1/Readme.md>)
- [Page 2](<Page 2/Readme.md>)

---

Enjoy clicking through the navigation :)

The markdown folder structure that needs to be followed is as follows:

1. Each folder should contain only one Readme.md file.
2. All other markdown files should be subfiles of this Readme.
3. To navigate to a lower level, create a folder and add another Readme.md file.

This structure is aligned with the organization of the [Collectives App in Nextcloud](https://github.com/nextcloud/collectives).

Currently the setup just works for the sppeling `Readme.md` for a Readme file (not README.md).

_See structure example:_

```
├── Page 1/
│   ├── Page 1 1/
│   │   ├── Readme.md
│   │   ├── example 1 1 1.md
│   │   ├── example 1 1 2.md
│   │   ├── example 1 1 3.md
│   │   ├── example 1 1 4.md
│   │  ──
│   ├── Readme.md
│   ├── example 1 1.md
│   ├── example 1 2.md
│   ├── example 1 3.md
│   ├── example 1 4.md
│  ──
├── Page 2/
│   ├── Page 2 1/
│   │   ├── Readme.md
│   │   ├── example 2 1 1.md
│   │   ├── example 2 1 2.md
│   │   ├── example 2 1 3.md
│   │   ├── example 2 1 4.md
│   │  ──
│   ├── Page 2 2/
│   │   ├── Readme.md
│   │   ├── example 2 2 1.md
│   │   ├── example 2 2 2.md
│   │   ├── example 2 2 3.md
│   │   ├── example 2 2 4.md
│   │  ──
│   ├── Readme.md
│   ├── example 1.md
│   ├── example 2.md
│   ├── example 3.md
│   ├── example 4.md
│  ──
├── Readme.md
├── example 1.md
```
