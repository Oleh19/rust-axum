struct Task {
    id: u32,
    title: String,
    done: bool
}

fn main() {
    let mut first = Task {
        id: 1,
        title: "Something".to_string(),
        done: false,
    };

    first.done = true;

    println!("{} is done: {}", first.title, first.done);

    print_title(&first);
    print_title(&first);

    let maybe_title: Option<String> = Some("Buy milk".to_string());

    match maybe_title {
        Some(t) => println!("Title: {}", t),
        None => println!("No title found"),
    }

    let parse: Result<u32, _> = "42".parse::<u32>();
    match parse {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error parsing number: {}", e),
    }
}

fn print_title(task: &Task) {
    println!("Task title: {}", task.title);
}
