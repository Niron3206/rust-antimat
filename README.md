
![Bad_words](https://cdn.discordapp.com/attachments/695563421491396728/1121958530551922769/rustacean-badwords.png)

**English** | [Русский](./README-ru.md)

# 🤬 rust-antimat
**rust-antimat** — discord bot that deletes messages by their inappropriate content. It uses [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) algorithm to define percentage of similarity with given list of words from `bad_words.txt` file. It has a message logging system, though.

**Initially, it supports english and russian swears filtering, but you can add your words in `bad_words.txt`**

## 🔧 Set up
1. Create your own bot here: [Discord Applications](https://discord.com/developers/applications)
2. Copy and paste your token in `.env` file. (Also, you can pick your own deleting percentage)
3. Run in cmd/terminal: `cargo.exe run --package rust-antimat --bin rust-antimat`