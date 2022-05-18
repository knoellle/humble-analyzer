# Humble Bundle Library Analyzer

A tool to convert json files as provided by the humble bundle website into a tab separated list that can be pasted into spreadsheet software.

## How to use

1. Install rust
2. Go to [https://www.humblebundle.com/home/keys](https://www.humblebundle.com/home/keys)
3. Open the network tab in your browsers developer console and refresh the page
4. Download all the `orders` json files and place them in `data/orders`. Filenames don't matter, but they have to use the `.json` extension.
5. `cargo run`
6. ???
7. Profit (Copy the output and paste it into your favorite spreadsheet software)

## Output format

Three tab separated columns:
- Name
- Steam store page link (if exists
- 0 if the key has not been claimed yet, i.e. revealed on the humble bundle page, does not take into account whether it has been used on steam, 1 otherwise.
