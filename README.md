# gomodes
Helpers ease `go mod` usage 

## recmd
Run command recursively on all go mod directories under current directory.

### Example
with this directory tree, run `recmd go mod tidy`, will run `go mod tidy` on directory modA, modB,  modC and modD.
```bash
├── modA
│    └── go.mod
├── modB
│    └── go.mod
└── modC
      ├── go.mod
      └── modD
            └── go.mod
```