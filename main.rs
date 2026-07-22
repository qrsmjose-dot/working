use std::rc::{Rc, Weak};
use std::cell::RefCell;
//It tells rust to import a specific smart pointer

#[derive(Clone, Copy)]//tells rust to duplicate switch without complaining
enum Status { Pending, Completed }
//2 way switch between Pending and Completed

struct Logger(Vec<String>); // Simple wrapper around a list of string logs
//Holds a list of log messages

struct Task {//starts def of task
    name: String,//tasks title as text "Build API"
    status: RefCell<Status>,//the switch of pend or comp so it can be changed later
    parent: RefCell<Option<Weak<Task>>>,//safe link to parents task
    deps: RefCell<Vec<Rc<Task>>>,//lets multiple parents share the same task
    logger: Rc<RefCell<Logger>>,//shared key to access central log book
}

impl Task {//defining fuctions for task
    fn new(name: &str, logger: Rc<RefCell<Logger>>) -> Rc<Self> {//takes shared name and logger and wraps new task in rc
        Rc::new(Self {//builds task and puts in shared rc key
            name: name.to_string(),//makes a permanent owned copy of the name string
            status: RefCell::new(Status::Pending),//sets pend statys in refcell so it can be switched later
            parent: RefCell::new(None),//starts with no parent so it can be attached to one later
            deps: RefCell::new(Vec::new()),//starts empty subtask list so subtask can be attached later
    
            logger,//stores shared logger key
        })
    }
}

fn add_dependency(parent: &Rc<Task>, child: &Rc<Task>) {//takes parent and child task and link them tgth
    parent.deps.borrow_mut().push(Rc::clone(child));//open parents subtask key and adds strong key(rc)
    *child.parent.borrow_mut() = Some(Rc::downgrade(parent));//demote parents rc in weak and give to child
}

fn complete_task(task: &Rc<Task>) {//marks a finished task and upd logs
    *task.status.borrow_mut() = Status::Completed;//open task status and flip to completed
    let mut log = task.logger.borrow_mut();//open share logbook so we can write inside
    log.0.push(format!("Task '{}' completed.", task.name));//writes take name completed in logbook

    // Safely upgrade the Weak pointer and log if parent still exists
    if let Some(p) = task.parent.borrow().as_ref().and_then(|w| w.upgrade()) {//checks if parent exist and pgrd to active key
        log.0.push(format!("Notified parent '{}'.", p.name));//if parent still alive write notified(parents name)
    }
}

fn main() {//main entry point where program start running
    let logger = Rc::new(RefCell::new(Logger(vec![]))); //creates empty logbook in refcell so it can be edited
    
    let parent = Task::new("Build API", Rc::clone(&logger));//Creates the "Build API" parent task and hands it a key to the shared logger.
    let child = Task::new("Setup DB", Rc::clone(&logger));//Creates the "Setup DB" child task and hands it a key to the shared logger.

    add_dependency(&parent, &child);//Links them together (Parent gets key to Child; Child gets business card to Parent).
    complete_task(&child);//Marks "Setup DB" as completed, writes to the logbook, and notifies "Build API"

    // Print all logged messages
    for msg in &logger.borrow().0 {
        println!("{msg}");
    }
}