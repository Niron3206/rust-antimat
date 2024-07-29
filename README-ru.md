
![Bad_words](https://github.com/Niron3206/rust-antimat/raw/master/rustacean-badwords.png)

[English](./README.md) | **Русский**

# 🤬 rust-antimat
**rust-antimat** — дискорд бот, который удаляет сообщения по их неприличному содержанию. Бот использует [Расстояние Левенштейна](https://ru.wikipedia.org/wiki/Расстояние_Левенштейна) для определения процетной схожести с заданными словами из `bad_words.txt` файла. Бот также имеет функцию логирования сообщений.

**Изначально, бот поддерживает фильтрацию английских и русских матных слов, но вы можете это изменить, добавив своих слов в `bad_words.txt`**

## 🔧 Set up
1. Создайте своего бота здесь: [Discord Applications](https://discord.com/developers/applications)
2. Копируйте и вставьте свой токен в `.env` файл. (В добавок, вы можете указать свой процентный порог)
3. Введите в командную строку/терминал: `cargo run --package rust-antimat --bin rust-antimat`
