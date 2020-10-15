# Rosa parse

Rosa parse is a command line utility to convert json, yaml or toml from stdin to json, yaml or toml
to stdout.

## Installation

```
cargo install rosa-parse
```

## Usage

Rosa parse is composed of three binaries :`to-toml`, `to-yaml`, `to-json`. 

They take no arguments and expect json, yaml or toml from stdin. 

Assuming we have the following json file : 
```json
{
    "fruits": [
        "apple",
        "banana"
    ]
}
```

You could convert it to any format : 
- to toml : 
    ```
    ❯ to-toml < fruits.json
    fruits = ["apple", "banana"]
    ```
  
- to yaml : 
    ```
    ❯ to-yaml < fruits.json
    ---
    fruits:
      - apple
      - banana
    ```
 - back to json : 
     ```
     ❯ to-json < fruits.json               
     {
       "fruits": [
         "apple",
         "banana"
       ]
     }
     ```
 - to yaml and then to toml
    ```
    ❯ cat fruits.json | to-yaml | to-toml 
    fruits = ["apple", "banana"]
    ```

## Licence

All the code in this repository is released under the MIT License, for more information take a look at the [LICENSE](LICENSE) file.