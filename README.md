![Makeit thumbnail](assets/thumbnail-small.png)

# makeit

Utility for creating and loading templates

## Table of Contents
- [Installation](#installation)
    - [AUR package](#aur-package)
    - [Compile it your own](#compile-it-your-own)
- [Usage](#usage)
    - [Loading templates](#loading-templates)
    - [Creating template](#creating-template)
    - [Other usage](#other-usage)
- [Detailed description](#detailed-description)
    - [Custom expression language](#custom-expression-language)
        - [Variables](#variables)
            - [Internal variables](#internal-variables)
        - [Literals](#literals)
        - [Operators](#operators)
            - [Operator +](#operator-)
            - [Operator ==](#operator-)
            - [Operator ??](#operator-)
- [Technologies](#technologies)
- [Links](#links)

## Installation

### AUR package

`makeit` is available as an
[AUR package](https://aur.archlinux.org/packages/makeit). You can install it
with any AUR package manager. This is example installation with
[`yay`](https://github.com/Jguer/yay):

```
yay -S makeit
```

### Compile it your own

You can also clone this repo and compile it yourself. But that shouldn't be a
problem, since only thing you need is `cargo`:
```
cargo build -r
```
After it's done compiling, you can start it in `./target/release/makeit`

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
To see full usage and other options, visit `makeit` help or `man-page`:
```
./makeit -h
```

## Detailed description

### Custom expression language
For parametrization of the templates I created custom expression language.
Expressions are enclosed in `{{` and `}}`.

#### Variables
- Can be defined in `makeit.json` file of the template or supplied using
command-line arguments
- Name has to start with alphabetic character or underscore and is followed
by any alphanumeric character or underscore

##### Internal variables
- `_PNAME`: project name based on project directory
- `_PDIR`: project directory
- `_OS`: operatins system

#### Literals
- Enclosed in double quotes (")
- They support escape sequences:
    - `\n`: newline
    - `\r`: carriage return
    - `\t`: tabulator
    - `\\`: backslash
    - `\"`: double quotes
    - Other sequences are expanded to character following backslash

#### Operators

##### Operator +
- Variables and literals concatenation
- Combines them to single literal
- Syntax:
    - `EXPR1 + EXPR2`

##### Operator ==
- Compares two values for equality (`true` when equals, else `false`)
- Syntax:
    - `EXPR1 == EXPR2`

##### Operator ??
- The null coalescing operator - provides default value for an expression,
which evaluates to `null`
- Syntax:
    - `EXPR1 ?? EXPR2`: returns value of `EXPR1` of not `null` else value of
    `EXPR2`

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
