use chrono;

use regex::Regex;
use serenity::model::user::User;
use std::cmp::{max, min};
//use std::collections::HashMap;
use std::{fs, env};

pub fn extract_regular_chars(text: &str) -> String {
    let re = Regex::new(r"[^a-zA-Zа-яА-Я0-9\s]").unwrap();
    let regular_chars = re.replace_all(text, "");
    regular_chars.into_owned()
}

pub fn is_bad(target: &str, source: &Vec<String>, author: &User) -> bool {
    let percentage = env::var("percantage").unwrap().parse::<usize>().unwrap();

    for word in source.into_iter() {
        let ratio = lev(target, &word) as usize;

        if ratio >= percentage {
            let log = format!("Автор сообщения — {}\nИзначальное слово — {}\nПроверочное слово — {}\nПроцентная схожесть — {}%",
                                         author.tag(), target, &word, ratio);

            let path = format!("./logs/{}.txt", chrono::offset::Local::now().format("%Y-%m-%d_%H.%M.%S"));
            fs::write(path, log).expect("Не удалось логировать событие");
            return true;
        }
    }

    false
}

/*
pub fn replace_engletters(text: &str) -> String {

    let mut text = String::from(text);

    let letters = HashMap::from([
        ("a", "а"),
        ("b", "в"),
        ("e", "е"),
        ("k", "к"),
        ("m", "м"),
        ("h", "н"),
        ("o", "о"),
        ("p", "р"),
        ("c", "с"),
        ("t", "т"),
        ("y", "у"),
        ("x", "х"),
    ]);

    for (en, ru) in letters.iter() {
        text = text.replace(en, ru);
    }

    text

}
 */


fn lev(target: &str, source: &str) -> f64 {

    let len_target = target.chars().count();
    let len_source = source.chars().count();

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; len_target + 1]; len_source + 1];
    
    for i in 1..=len_source {
        matrix[i][0] = i;
    }
    for i in 1..=len_target {
        matrix[0][i] = i;
    }

    for (i, source_chars) in source.chars().enumerate() {
        for (j, target_chars) in target.chars().enumerate() {
            let cost = if source_chars == target_chars {0} else {1};
            matrix[i+1][j+1] = min(min(
                matrix[i][j+1] + 1,
                matrix[i+1][j] + 1
            ), matrix[i][j] + cost);
        }
    }

    let lev = matrix[len_source][len_target] as f64;
    let max = max(len_target, len_source) as f64;
    
    ((max - lev) / max) * 100.

}
