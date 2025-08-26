use std::collections::HashMap;
use std::io;

fn main() {
    fn find_mid_and_freq(arr: &Vec<i32>) -> HashMap<String, f32> {
        let mut sorted = arr.clone();
        sorted.sort();

        let len = sorted.len();
        let mut result: HashMap<String, f32> = HashMap::new();

        if len % 2 == 0 {
            let mid1 = sorted[len / 2 - 1];
            let mid2 = sorted[len / 2];
            result.insert(String::from("mid"), (mid1 as f32 + mid2 as f32) / 2.0);
        } else {
            result.insert(String::from("mid"), sorted[len / 2] as f32);
        }

        let mut freq: HashMap<i32, i32> = HashMap::new();

        for &i in arr {
            let count = freq.entry(i).or_insert(0);

            *count += 1;
        }

        let freq_num = freq
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("No frequency found");

        result.insert(String::from("freq"), freq_num as f32);

        return result;
    }

    let numbers = vec![1, 3, 8, 4, 5, 9, 4, 6, 1, 2, 4];
    println!("{:?}", find_mid_and_freq(&numbers));

    fn change_to_pig_latin(s: &str) -> String {
        let yuan = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        if let Some(first_char) = s.chars().next() {
            if yuan.contains(&first_char) {
                format!("{}-hay", s)
            } else {
                let mut chars = s.chars();
                let first = chars.next().unwrap();
                let rest: String = chars.collect();

                format!("{}-{}ay", rest, first)
            }
        } else {
            String::new()
        }
    }

    let words = [
        "first", "second", "third", "forth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth",
    ];

    for word in words {
        println!("{} change to {}", word, change_to_pig_latin(word));
    }

    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    println!("employee manangment system");

    loop {
        println!("please enter command(enter 'quit' exist):");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("input failed!");

        let input = input.trim();

        if input == "quit" {
            break;
        }

        excute_commend(input, &mut deps);
    }

    fn excute_commend(cmd: &str, deps: &mut HashMap<String, Vec<String>>) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        if parts.len() < 2 {
            println!("cmd is unvaild");
            return;
        }
        match parts[0] {
            "add" => {
                if parts.len() >= 4 && parts[parts.len() - 2] == "to" {
                    let name = parts[1..parts.len() - 2].join(" ");
                    let dep = parts.last().unwrap().to_string();

                    add_emp(&name, &dep, deps);
                    println!("add {} to {}", name, dep);
                } else {
                    println!(" CMD add error, should be add <name> to <dep>")
                }
            }
            "list" => {
                if parts[1] == "all" {
                    list_all_dep(deps);
                } else {
                    let dep = parts[1..].join(" ");
                    list_dep(&dep, deps);
                }
            }
            _ => println!("unknown cmd"),
        }
    }

    fn add_emp(name: &str, dep: &str, deps: &mut HashMap<String, Vec<String>>) {
        let emp = deps.entry(dep.to_string()).or_insert(Vec::new());
        emp.push(name.to_string());
        emp.sort();
    }

    fn list_dep(dep: &str, deps: &mut HashMap<String, Vec<String>>) {
        if let Some(emps) = deps.get(dep) {
            print!("{dep}'s employee");
            for emp in emps {
                println!(" - {emp}");
            }
        } else {
            println!("{dep} does not exist");
        }
    }

    fn list_all_dep(deps: &mut HashMap<String, Vec<String>>) {
        if deps.is_empty() {
            println!("no data");
            return;
        }

        let mut dep_list: Vec<&String> = deps.keys().collect();
        dep_list.sort();

        println!("list");
        for dep in dep_list {
            println!("{dep}:");
            if let Some(emps) = deps.get(dep) {
                for emp in emps {
                    println!("  - {emp}");
                }
            }
        }
    }
}
