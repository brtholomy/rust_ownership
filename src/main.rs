#![allow(dead_code)]

// Ownership.
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// 1. either one mutable reference or any number of immutable references.

// 2. references must always be valid.

////////////////////////////////////////
// moving within scope

fn cloning() {
    let s3 = String::from("foo ");
    let mut s4 = s3.clone();
    s4.push_str("cloned!");
    println!("cloning: {:?}", s4);
}

fn mut_move() {
    // mut move. s is no longer valid.
    let s = String::from("wha? ");
    let mut s2 = s;
    s2.push_str(" fool");
    println!("mut_move: {:?}", s2);
    // let err = s;
}

////////////////////////////////////////
// function moves

// simplest case. immut by default:
fn simple_move(s: String) {
    println!("simple_move: {:?}", s);
}

// move from inner scope
fn move_from_inner() -> String {
    let s = String::from("inner");
    return s;
}

// move and move back. immutable.
fn move_in_and_out(s: String) -> String {
    println!("move_in_and_out : {:?}", s.len());
    return s;
}

// move and move back. here mut
fn move_in_and_out_mut(mut s: String) -> String {
    s.push_str(" move_in_and_out_mut!");
    return s;
}

////////////////////////////////////////
// references

// Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
// We call the action of creating a reference borrowing.
fn take_ref(s: &String) -> usize {
    return s.len();
}

fn take_mut_ref(s: &mut String) {
    s.push_str("mutable ref");
}

fn multiple_refs() {
    let s = String::from("foo");
    let r1 = &s;
    let r2 = &s;
    println!("r1, r2: {:?}", (r1, r2));
}

fn multiple_mut_refs() {
    let mut s = String::from("foo");

    {
        let r1 = &mut s;
        r1.push_str(" changed");
        println!("r1: {:?}", r1);
    }
    let r2 = &s;
    println!("r2: {:?}", r2);
}

fn multiple_mut_refs_scope() {
    let mut s = String::from("foo");

    let r2 = &s;
    println!("r2: {:?}", r2);

    // this is okay, because the compiler sees no usage of r2
    // after the print. seems hacky. not a good idea to depend on ordering.
    let r1 = &mut s;
    r1.push_str(" changed");
    println!("r1: {:?}", r1);
}

// returns a reference to data owned by the current function
// fn dangler() -> &'static String {
//     let s = String::from("blah");
//     return &s;
// }

fn main() {
    ////////////////////////////////////////
    // these are moves within scope:
    cloning();
    mut_move();

    ////////////////////////////////////////
    // these are function moves:
    let s = String::from("neat.");
    simple_move(s);

    println!("move_from_inner(): {:?}", move_from_inner());

    // taking immut value
    let s2 = String::from("move in and out");
    let s3 = move_in_and_out(s2);
    println!("s3: {:?}", s3);

    // taking mut value
    let s5 = String::from("foo");
    let s6 = move_in_and_out_mut(s5);
    println!("s6: {:?}", s6);

    let length = take_ref(&s6);
    println!("take_ref: {:?}", length);

    let mut s7 = String::from("blah");
    take_mut_ref(&mut s7);
    println!("take_mut_ref: {:?}", s7);

    ////////////////////////////////////////
    multiple_refs();
    multiple_mut_refs();
    multiple_mut_refs_scope();
}
