use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Clone, Copy)]
enum Status { Pending, Completed }

struct Logger(Vec<String>); // Simple wrapper around a list of string logs

struct Task {
    name: String,
    status: RefCell<Status>,
    parent: RefCell<Option<Weak<Task>>>,
    deps: RefCell<Vec<Rc<Task>>>,
    logger: Rc<RefCell<Logger>>,
}

impl Task {
    fn new(name: &str, logger: Rc<RefCell<Logger>>) -> Rc<Self> {
        Rc::new(Self {
            name: name.to_string(),
            status: RefCell::new(Status::Pending),
            parent: RefCell::new(None),
            deps: RefCell::new(Vec::new()),
            logger,
        })
    }
}

fn add_dependency(parent: &Rc<Task>, child: &Rc<Task>) {
    parent.deps.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Some(Rc::downgrade(parent));
}

fn complete_task(task: &Rc<Task>) {
    *task.status.borrow_mut() = Status::Completed;
    let mut log = task.logger.borrow_mut();
    log.0.push(format!("Task '{}' completed.", task.name));

    // Safely upgrade the Weak pointer and log if parent still exists
    if let Some(p) = task.parent.borrow().as_ref().and_then(|w| w.upgrade()) {
        log.0.push(format!("Notified parent '{}'.", p.name));
    }
}

fn main() {
    let logger = Rc::new(RefCell::new(Logger(vec![])));
    
    let parent = Task::new("Build API", Rc::clone(&logger));
    let child = Task::new("Setup DB", Rc::clone(&logger));

    add_dependency(&parent, &child);
    complete_task(&child);

    // Print all logged messages
    for msg in &logger.borrow().0 {
        println!("{msg}");
    }
}