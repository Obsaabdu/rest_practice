use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug)]
struct Task {
    title: String,
    completed: bool,
    tags: Vec<String>,
}

#[derive(Debug)]
struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

#[derive(Debug)]
struct TaskStatistics {
    total: usize,
    completed: usize,
    pending: usize,
    total_tags: usize,
}

enum TaskCompleteStatus {
    Completed,
    AlreadyCompleted,
    NotFound,
}
enum TaskDeleteStatus {
    Deleted,
    NotFound,
}
enum TaskRenameStatus {
    Renamed,
    Unchanged,
    NotFound,
}
enum TaskTagAddStatus {
    Added,
    TaskNotFound,
    TagDuplicate,
}
enum TaskTagRemoveStatus {
    Removed,
    TaskNotFound,
    TagNotFound,
}

impl Task {
    fn new<S: Into<String>>(title: S, tags: Vec<String>) -> Self {
        let title = title.into();
        Task {
            title,
            completed: false,
            tags,
        }
    }
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task<S: Into<String>>(&mut self, title: S) {
        let task = Task::new(title.into(), Vec::new());
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    fn add_tag<S: Into<String>>(&mut self, id: u32, tag: S) -> TaskTagAddStatus {
        let new_tag = tag.into();
        match self.tasks.get_mut(&id) {
            Some(task) => {
                if task.tags.contains(&new_tag) {
                    TaskTagAddStatus::TagDuplicate
                } else {
                    task.tags.push(new_tag);
                    TaskTagAddStatus::Added
                }
            }
            None => TaskTagAddStatus::TaskNotFound,
        }
    }

    fn remove_tag<S: Into<String>>(&mut self, id: u32, tag: S) -> TaskTagRemoveStatus {
        let removed_tag = tag.into();
        match self.tasks.get_mut(&id) {
            Some(task) => {
                let original_len = task.tags.len();
                task.tags.retain(|t| t != &removed_tag);
                if task.tags.len() != original_len {
                    TaskTagRemoveStatus::Removed
                } else {
                    TaskTagRemoveStatus::TagNotFound
                }
            }
            None => TaskTagRemoveStatus::TaskNotFound,
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("(no tasks added)");
            return;
        }
        for (id, task) in &self.tasks {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{} #{} - {}", status, id, task.title);
            if !task.tags.is_empty() {
                println!("tags: {}", task.tags.join(", "));
            }
        }
    }

    fn filter_tasks_by_tag(&self, tag: &String) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.tags.contains(tag))
            .collect()
    }

    fn rename_task<S: Into<String>>(&mut self, id: u32, new_title: S) -> TaskRenameStatus {
        let new_title = new_title.into();
        match self.tasks.get_mut(&id) {
            Some(task) => {
                if task.title == new_title {
                    TaskRenameStatus::Unchanged
                } else {
                    task.title = new_title;
                    TaskRenameStatus::Renamed
                }
            }
            None => TaskRenameStatus::NotFound,
        }
    }

    fn complete_task(&mut self, id: u32) -> TaskCompleteStatus {
        if let Some(task) = self.tasks.get_mut(&id) {
            if task.completed {
                return TaskCompleteStatus::AlreadyCompleted;
            }
            task.completed = true;
            TaskCompleteStatus::Completed
        } else {
            TaskCompleteStatus::NotFound
        }
    }

    fn delete_task(&mut self, id: u32) -> TaskDeleteStatus {
        match self.tasks.remove(&id) {
            Some(_) => TaskDeleteStatus::Deleted,
            None => TaskDeleteStatus::NotFound,
        }
    }

    fn statistics(&self) -> TaskStatistics {
        let total = self.tasks.len();
        let completed = self.tasks.values().filter(|task| task.completed).count();
        let total_tags = self.tasks.values().map(|task| task.tags.len()).sum();
        TaskStatistics {
            total,
            completed,
            pending: total - completed,
            total_tags,
        }
    }
}

fn prompt(text: &str) -> String {
    loop {
        print!("{text}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let response = input.trim();
        if response.is_empty() {
            println!("the input shouldn't be empty or spaces only");
            continue;
        }
        return response.to_string();
    }
}
fn menu() {
    println!("===== Task Manager =====");
    println!("1. Add Task");
    println!("2. List Tasks");
    println!("3. Complete Task");
    println!("4. Delete Task");
    println!("5. Search Task");
    println!("6. Show Statistics");
    println!("7. Exit");
}

fn main() {
    let mut manager = TaskManager::new();
    manager.add_task("title");
    manager.add_tag(1, "tag");
    println!("{:?}", manager.statistics())
}
