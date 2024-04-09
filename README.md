# makeit

Utility for creating and loading templates

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
    - [Loading templates](#loading-templates)
    - [Creating template](#creating-template)
    - [Other usage](#other-usage)
- [Technologies](#technologies)
- [Links](#links)

## Installation
You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:
```
cargo build -r
```
After its done compiling, you can start it in `./target/release/makeit`

## Usage

### Loading templates
You can load already existing template. If you don't specify `-d`, template
will be loaded to current directory by default:
```
./makeit <template name> [-d load/template/to]
```

### Creating template
To create template you have to do this (note that if `-d` isn't specified,
template is create from current directory):
```
./makeit <template name> -c [-d create/template/from]
```

### Other usage
To see other options, visit `makeit` help:
```
./makeit -h
```

## Technologies
I used these libraries, which were really helpful:
- [dirs](https://crates.io/crates/dirs)
    - Accessing config folder
- [serde](https://crates.io/crates/serde)
    - Saving and loading to JSON
- [termint](https://crates.io/crates/termint)
    - Colored printing

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [termint](https://github.com/Martan03/makeit)
- **Author website:** [martan03.github.io](https://martan03.github.io)
