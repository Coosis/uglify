use std::collections::HashMap;
use regex::Regex;

fn prepend(text: &mut String, s: &str) {
    *text = format!("{}{}", s, text);
}

pub fn preprocess(text: &String) -> (String, String) {
    let mut header = String::new();
    let mut res = String::new();
    let safe = Regex::new(r"#include <[\w\.]+>").unwrap();
    let safe2 = Regex::new(r"#define [\w\.]+ [\w\.]+").unwrap();
    for l in text.lines() {
        if safe.is_match(l) {
            header.push_str(&format!("{}\n", l));
            continue;
        }
        if safe2.is_match(l) {
            header.push_str(&format!("{}\n", l));
            continue;
        }
        // println!("pushed {}", l);
        res.push_str(&format!("{}\n", l));
    }
    (header, res)
}

pub fn prepend_macro(text: &mut String, mp: &HashMap<String, i32>) {
    for (k, v) in mp {
        let mut torep = String::new();
        for _ in 0..*v {
            torep.push('_');
        }
        prepend(text, &format!("#define {} {}\n", torep, k));
    }
}

pub fn populate(text: &str, mp: &mut HashMap<String, i32>) {
    mp.insert("<<".to_string(), 1);
    let re = Regex::new(r#"("[^"]*"|\w+)"#).unwrap();
    let mut i: i32 = 2;
    for l in text.lines() {
        for (_, [word]) in re.captures_iter(l).map(|c| c.extract()) {
            let tmp = word.to_string();
            if mp.contains_key(&tmp) {
                continue;
            }
            mp.insert(tmp, i);
            i += 1;
            // println!("i is {}", i);
        }
    }
}

pub fn replace(text: &mut String, mp: &HashMap<String, i32>) {
    let (mut header, mut body) = preprocess(&text);
    let mut tos: Vec<_> = mp.iter().collect();
    tos.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    for (k, v) in tos {
        // println!("{}: {}", k, v);
        let mut torep = String::new();
        for _ in 0..*v {
            torep.push('_');
        }
        body = body.replace(k, &torep);
    }
    header.push_str(&body);
    *text = header;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn populate_test1() {
        let mut mp: HashMap<String, i32> = HashMap::new();
        let article = "int main() {\n\tcout << \"Hello World!\" << endl;\n}";
        populate(&article, &mut mp);
        assert!(mp.contains_key("int"));
        assert!(mp.contains_key("main"));
        assert!(mp.contains_key("cout"));
        assert!(mp.contains_key("\"Hello World!\""));
        assert!(mp.contains_key("endl"));
    }

    #[test]
    fn pupulate_test2() {
        let mut mp: HashMap<String, i32> = HashMap::new();
        let article = r#"""
        #include <iostream>
        #include <string>
        using namespace std;

        int main() {
            cout << "Hello My Friend!" << endl;
        }
        """#;
        populate(&article, &mut mp);
        assert!(!mp.contains_key("#include"));
        assert!(!mp.contains_key("<iostream>"));
        assert!(!mp.contains_key("iostream"));
        assert!(!mp.contains_key("string"));
        assert!(mp.contains_key("int"));
        assert!(mp.contains_key("main"));
        assert!(mp.contains_key("cout"));
        assert!(mp.contains_key("\"Hello My Friend!\""));
        assert!(mp.contains_key("endl"));
    }

    #[test]
    fn replace_test1() {
        let mut mp: HashMap<String, i32> = HashMap::new();
        let mut article = String::from(r#"
        #include <iostream>
        #include <string>
        using namespace std;

        int main() {
            cout << "Hello My Friend!" << endl;
        }
        "#);
        populate(&article, &mut mp);
        replace(&mut article, &mp);
        prepend_macro(&mut article, &mp);
        print!("{}", article);
    }
}
