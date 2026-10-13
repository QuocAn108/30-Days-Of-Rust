use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn ex2() {
    let mut tasks: Vec<Task> = Vec::new();
    tasks.push(Task {
        description: String::from("Task 1"),
        is_completed: false,
    });
    tasks.push(Task {
        description: String::from("Task 2"),
        is_completed: true,
    });
    tasks.push(Task {
        description: String::from("Task 3"),
        is_completed: false,
    });
    let mut tasks_person: VecDeque<Task> = VecDeque::new();
    tasks_person.push_front(Task {
        description: String::from("Task 1"),
        is_completed: false,
    });
    tasks_person.push_back(Task {
        description: String::from("Task 2"),
        is_completed: true,
    });
    tasks_person.push_front(Task {
        description: String::from("Task 3"),
        is_completed: true,
    });
    if let Some(front) = tasks_person.pop_front() {
        println!("Removed from front: {:?}", front);
    }
    for i in tasks_person {
        println!("{:?},", i);
    }

    print_list_tasks(tasks);
    let sentence = "An dep trai nhat the gioi do an co biet khong?";
    print_word_count_frenquency(sentence);
    let mut student = BTreeMap::new();
    student.insert("Ryan", 55);
    student.insert("An", 48);
    student.insert("Cyan", 30);
    for (name, old) in &student {
        println!("{}: {}", name, old);
    }
}

fn print_list_tasks(task: Vec<Task>) {
    for subtask in task {
        println!("Task: {}", subtask.description);
        println!(
            "Status: {}",
            if subtask.is_completed {
                "Done"
            } else {
                "Not Done"
            }
        )
    }
}
#[derive(Debug)]
struct Task {
    description: String,
    is_completed: bool,
}

fn print_word_count_frenquency(sentence: &str) {
    let mut collection: HashMap<&str, i32> = HashMap::new();
    let lower = sentence.to_lowercase();
    for word in lower.split_whitespace() {
        let count = collection.entry(word).or_insert(0);
        *count += 1;
    }
    for (word, count) in &collection {
        println!("{}: {}", word, count);
    }
}
