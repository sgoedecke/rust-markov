# Markov

Writing a markov chain text generator in Rust as a learning exercise. So far it's a _lot_ of type-juggling due to calls that could fail returning `Option` wrappers.

## Usage

`cargo run samplefile.txt` will consume a text file and produce a generated sentence

## Todo

One of the unwrap calls is failing, probably because I'm not stripping the `\n` off the end of the file.
Extract the work into functions.
Clean up `unwrap` calls.
