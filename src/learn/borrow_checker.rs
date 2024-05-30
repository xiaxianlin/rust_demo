use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Weak;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Item {
    pub contents: i32,
}

fn overwrite(item: &mut Option<Item>, val: Item) {
    *item = Some(val);
}

fn replace(item: &mut Option<Item>, val: Item) -> Option<Item> {
    // let previous = *item; // move out
    // *item = Some(val); // replace
    // previous
    std::mem::replace(item, Some(val))
}

fn replace_item(item: &mut Option<Item>, val: Item) -> Option<Item> {
    item.replace(val)
}

fn both_zero(left: &Item, right: &Item) -> bool {
    left.contents == 0 && right.contents == 0
}

fn zero_both(left: &mut Item, right: &mut Item) {
    left.contents = 0;
    right.contents = 0;
}

fn find<'a, 'b>(a: &'a str, b: &'b str) -> Option<&'a str> {
    a.find(b).map(|i| &a[i..i + b.len()])
}

fn check_item(_: Option<&Item>) {}

fn test() {
    let item = Item { contents: 12 };
    let haystack = format!("{} to search", "Text");
    let found = find(&haystack, "ex");
    if let Some(text) = found {
        println!("Found '{}'!", text);
    }

    let x: Option<Rc<RefCell<Item>>> = Some(Rc::new(RefCell::new(Item { contents: 42 })));

    let x1: Option<&Rc<RefCell<Item>>> = x.as_ref();
    let x2: Option<std::cell::Ref<Item>> = x1.map(|r| r.borrow());
    // let x3: Option<&Item> = x2.map(|r| r.deref());
    match x2 {
        None => check_item(None),
        Some(r) => {
            let x3: &Item = r.deref();
            check_item(Some(x3));
        }
    }

    match x.as_ref().map(|r| r.borrow()) {
        None => check_item(None),
        Some(r) => check_item(Some(r.deref())),
    };
}

#[derive(Debug, Clone)]
struct CustomError(String);

impl CustomError {
    fn new(msg: &str) -> Self {
        CustomError(msg.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PhoneNumber(u32);

impl PhoneNumber {
    fn new(data: u32) -> Self {
        PhoneNumber(data)
    }
}

#[derive(Debug, Clone)]
struct Guest {
    name: String,
    phone: PhoneNumber,
}

#[derive(Default, Debug)]
struct GuestRegister(Vec<Guest>);

impl GuestRegister {
    fn register(&mut self, guest: Guest) {
        self.0.push(guest)
    }
    fn nth(&self, idx: usize) -> Option<&Guest> {
        if idx < self.0.len() {
            Some(&self.0[idx])
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
struct ClonedGuestRegister {
    by_arrival: Vec<Guest>,
    by_name: BTreeMap<String, Guest>,
}

impl ClonedGuestRegister {
    fn register(&mut self, guest: Guest) {
        self.by_arrival.push(guest.clone()); // requires `Guest` to be `Clone`
        self.by_name.insert(guest.name.clone(), guest);
    }
    fn named(&self, name: &str) -> Option<&Guest> {
        self.by_name.get(name)
    }
    fn named_mut(&mut self, name: &str) -> Option<&mut Guest> {
        self.by_name.get_mut(name)
    }
    fn deregister(&mut self, idx: usize) -> Result<(), CustomError> {
        if idx >= self.by_arrival.len() {
            return Err(CustomError::new("out of bounds"));
        }
        let guest = self.by_arrival.remove(idx);
        self.by_name.remove(&guest.name);
        Ok(())
    }
}

#[derive(Default, Debug)]
struct RcGuestRegister {
    by_arrival: Vec<Rc<RefCell<Guest>>>,
    by_name: BTreeMap<String, Rc<RefCell<Guest>>>,
}

impl RcGuestRegister {
    fn register(&mut self, guest: Guest) {
        let name = guest.name.clone();
        let guest = Rc::new(RefCell::new(guest));
        self.by_arrival.push(guest.clone());
        self.by_name.insert(name, guest);
    }
    fn deregister(&mut self, idx: usize) -> Result<(), CustomError> {
        if idx >= self.by_arrival.len() {
            return Err(CustomError::new("out of bounds"));
        }
        let guest = self.by_arrival.remove(idx);
        self.by_name.remove(&guest.borrow().name);
        Ok(())
    }
    fn named(&self, name: &str) -> Option<&Rc<RefCell<Guest>>> {
        self.by_name.get(name)
    }
    fn named_mut(&mut self, name: &str) -> Option<&mut Rc<RefCell<Guest>>> {
        self.by_name.get_mut(name)
    }
}

pub fn test1() {
    let mut ledger = RcGuestRegister {
        by_arrival: vec![],
        by_name: BTreeMap::new(),
    };

    let alice = Guest {
        name: String::from("Alice"),
        phone: PhoneNumber(1234),
    };

    let bob = Guest {
        name: String::from("Bob"),
        phone: PhoneNumber(2234),
    };

    let charlie = Guest {
        name: String::from("Charlie"),
        phone: PhoneNumber(3234),
    };

    ledger.register(alice);
    ledger.register(bob);
    ledger.register(charlie);
    println!("Register starts as: {:?}", ledger);

    let new_number = PhoneNumber::new(123456);
    ledger.named_mut("Bob").unwrap().borrow_mut().phone = new_number;
    assert_eq!(ledger.named("Bob").unwrap().borrow().phone, new_number);

    ledger.deregister(0).unwrap();
    println!("Register after deregister(0): {:?}", ledger);

    let also_alice = ledger.named("Alice");
    println!("Alice is {:?}", also_alice);

    let also_bob = ledger.named("Bob");
    println!("Bob is {:?}", also_bob);

    let also_charlie = ledger.named("Charlie");
    println!("Charlie is {:?}", also_charlie);
}

struct Tree {
    tree_id: String,
    branches: Vec<Rc<RefCell<Branch>>>,
}

struct Branch {
    branch_id: String,
    leaves: Vec<Rc<RefCell<Leaf>>>,
    owner: Option<Weak<RefCell<Tree>>>,
}

struct Leaf {
    leaf_id: String,
    owner: Option<Weak<RefCell<Branch>>>,
}

impl Tree {
    fn add_branch(&mut self, branch: Branch) {
        self.branches.push(Rc::new(RefCell::new(branch)));
    }
    fn id(&self) -> String {
        self.tree_id.clone()
    }
}

impl Branch {
    fn add_leaf(branch: Rc<RefCell<Branch>>, mut leaf: Leaf) {
        leaf.owner = Some(Rc::downgrade(&branch));
        branch.borrow_mut().leaves.push(Rc::new(RefCell::new(leaf)));
    }
    fn id(&self) -> String {
        match &self.owner {
            None => format!("<unowned>.{}", self.branch_id),
            Some(t) => {
                let tree = t.upgrade().expect("internal error: owner gone!");
                format!("{}.{}", tree.borrow().id(), self.branch_id)
            }
        }
    }
}
