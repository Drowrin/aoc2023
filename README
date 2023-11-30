<div align="center">
# Advent of Code 2023
My solutions in Rust!
</div>

## Usage
This repo uses [just](https://github.com/casey/just) to run common commands.

Since I assume you've got cargo:
```sh
cargo install just
```

You can then use the following commands:
```sh
# Initialize a project with a name based on <day>
just new <day>

# Same as above, but copies over example data, input data, and your previous solution
# Useful for part two of each day
just new <day> <from>

# Run your solution for a day on the example data, and compare to the example answer
just test <day>

# Run your solution on your unique input data, and output the answer
just run <day> <flags>

# Transfer example data, input data, and solution from one day to another
# Useful if you forgot to do `just new <day> <from>`
# Be careful! This will overwrite files in the `to` directory
just transfer <from> <to>
```

### Naming Convention
The `just` commands enforce a naming convention for "days". They must all be a 2 digit number from 01-25, followed by a single lowercase letter. This ensures the folders will sort nicely and are easy to work with. The letter at the end is useful for separating a day into two parts, such as "a" and "b".

If you want to change this behavior, edit the `validate` recipe in the `justfile`.

### Structure
Once you've made a project with `just new <day>`, you'll need to copy some data from Advent of Code. For all of these, avoid trailing newlines.
- In `data/answer.txt`, put the example answer.
- In `data/example.txt`, put the example input.
- In `data/input.txt`, put your unique input data.
- Write your solution code in `src/solution.rs`.
