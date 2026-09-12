// ============================================================================
// keywords_demo — every one of Rust's 39 active keywords, used at least once,
// organized around the 10 traits that make Rust distinctive.
//
// Trait key (referenced in comments below as [T1]..[T10]):
//   T1  Ownership & the borrow checker
//   T2  Expression-oriented design (almost everything returns a value)
//   T3  Zero-cost abstractions (high-level code, low-level speed)
//   T4  Traits as flexible interfaces
//   T5  Fearless concurrency
//   T6  Pattern matching
//   T7  Option/Result instead of null
//   T8  Macros
//   T9  Immutability by default
//   T10 The `?` operator for error propagation
//
// (Cargo/tooling was the 11th trait discussed — deliberately not shown here,
// since it's an ecosystem property, not something expressible in source code.)
// ============================================================================

use std::fmt;                              // [use]
use std::thread;                           // for the concurrency demo below

// ---------------------------------------------------------------------------
// [T4] Traits as flexible interfaces — `shapes` groups a trait + two structs
// that both implement it, the idiomatic Rust stand-in for class inheritance.
// ---------------------------------------------------------------------------
mod shapes {                               // [mod]
    pub trait Shape {                      // [pub] [trait]  -> [T4]
        fn area(&self) -> f64;             // [fn]  &self is a borrowed ref -> [T1]
    }

    pub struct Circle {                    // [struct]
        pub radius: f64,
    }

    pub struct Square {
        pub side: f64,
    }

    impl Shape for Circle {                // [impl]         -> [T4]
        fn area(&self) -> f64 {
            Self { radius: self.radius }.radius * self.radius * std::f64::consts::PI
            // ^ `Self` refers to the implementing type, `self` to this instance
            //   [Self]                              [self]   -> [T1]
        }
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    // A nested module, used only to give `super` something real to reach for.
    pub mod inner {
        use super::Shape;                  // [super] — refers to the parent module
        pub fn describe<T: Shape>(s: &T) -> String {
            format!("area = {:.2}", s.area())
        }
    }
}

use shapes::{Circle, Square, Shape as ShapeTrait};  // [use] ... [as]  -> renaming an import
use shapes::inner::describe;
use crate::shapes::Shape as _;              // [crate] — an absolute path from the crate root

// ---------------------------------------------------------------------------
// [T6] Pattern matching — a plain enum whose variants we `match` on later.
// ---------------------------------------------------------------------------
enum Status {                                // [enum]
    Active,
    Inactive,
}

// ---------------------------------------------------------------------------
// [T9] Immutability by default — `const` and `static` are Rust's two flavors
// of "never mutable," in contrast to `let mut` bindings used further below.
// ---------------------------------------------------------------------------
const MAX_ITEMS: usize = 5;                  // [const]  -> [T9]
static GREETING: &str = "Hello from Rust";   // [static] -> [T9]

// ---------------------------------------------------------------------------
// [T3] Zero-cost abstractions — a type alias over a trait-object collection.
// `dyn` marks it as dynamic dispatch; the iterator chain that uses this list
// (further down) compiles to the same speed as a hand-written loop.
// ---------------------------------------------------------------------------
type ShapeList = Vec<Box<dyn ShapeTrait>>;   // [type] ... [dyn] -> [T3] [T4]

// ---------------------------------------------------------------------------
// [T2] Expression-oriented design — this generic fn's `where` clause plus its
// `if`/`else` body both return values rather than acting as bare statements.
// ---------------------------------------------------------------------------
fn grade_for(score: u32) -> &'static str
where
    u32: PartialOrd,                         // [where]
{
    if score >= 90 { "A" } else { "B" }      // [if] [else] -> [T2]
}

// ---------------------------------------------------------------------------
// [T7] / [T10] Option & Result instead of null, propagated with `?`.
// ---------------------------------------------------------------------------
fn parse_radius(input: &str) -> Result<f64, std::num::ParseFloatError> {
    let value = input.parse::<f64>()?;       // [let]            -> [T10]
    Ok(value)
}

fn find_by_status(shapes_status: &[Status], target: bool) -> Option<&Status> {
    for s in shapes_status {                 // [for] ... [in]   -> [T6]-adjacent iteration
        match s {
            Status::Active if target => return Some(s),   // [match] [return] -> [T6]
            Status::Inactive if !target => return Some(s),
            _ => continue,                    // [_] [continue]
        }
    }
    None                                      // -> [T7]
}

// ---------------------------------------------------------------------------
// [T8] Macros — a small hygienic macro, invoked below.
// ---------------------------------------------------------------------------
macro_rules! trait_label {
    ($name:expr) => {
        println!(">> demonstrating: {}", $name);
    };
}

// ---------------------------------------------------------------------------
// [T5] Fearless concurrency — `move` hands ownership of `data` into the new
// thread; the borrow checker guarantees no other code can touch it after.
// ---------------------------------------------------------------------------
fn spawn_worker(data: Vec<i32>) -> thread::JoinHandle<i32> {
    thread::spawn(move || {                   // [move]           -> [T1] [T5]
        data.iter().sum()                     // iterator chain   -> [T3]
    })
}

// ---------------------------------------------------------------------------
// [T5] Async is Rust's other concurrency model, cooperative rather than
// OS-threaded. `async`/`await` here run on the tokio runtime.
// ---------------------------------------------------------------------------
async fn fetch_greeting() -> &'static str {   // [async]
    GREETING
}

async fn unsafe_ffi_example() -> i32 {        // [async]
    let raw: *const i32 = &42;
    let value = unsafe { *raw };               // [unsafe] — dereferencing a raw pointer
    value
}

extern "C" {                                  // [extern] — FFI declaration block
    fn abs(input: i32) -> i32;
}

fn call_c_abs(n: i32) -> i32 {
    unsafe { abs(n) }                          // [unsafe] again, required at every FFI call site
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Active => write!(f, "Active"),
            Status::Inactive => write!(f, "Inactive"),
        }
    }
}

fn print_all<T: fmt::Display>(items: &[T]) {
    for item in items {                        // [for] [in]
        println!("{}", item);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {                              // [async]          -> [T5]
    trait_label!("T4 traits as interfaces, T1 ownership");

    let circle = Circle { radius: 2.0 };       // [let]            -> [T9] immutable by default
    let square = Square { side: 3.0 };
    println!("{}", describe(&circle));
    println!("{}", describe(&square));

    let shapes: ShapeList = vec![Box::new(Circle { radius: 1.0 }), Box::new(Square { side: 2.0 })];
    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();  // -> [T3] zero-cost abstraction
    println!("total area = {:.2}", total_area);

    trait_label!("T2 expression-oriented design");
    let grade = grade_for(95);                 // if/else as an expression -> [T2]
    println!("grade = {}", grade);

    trait_label!("T7 and T10: Option/Result plus the ? operator");
    match parse_radius("4.5") {
        Ok(r) => println!("parsed radius = {}", r),
        Err(_) => println!("parse failed"),
    }

    trait_label!("T6 pattern matching");
    let statuses = vec![Status::Active, Status::Inactive];
    if let Some(found) = find_by_status(&statuses, true) {
        println!("found status: {}", found);
    }
    print_all(&statuses);

    trait_label!("T9 immutability by default, then an explicit mut");
    let ref count_ref = MAX_ITEMS;             // [ref] — explicitly bind by reference
    println!("MAX_ITEMS via ref binding = {}", count_ref);

    let mut counter: usize = 0;                // [mut] — the deliberate opt-out of T9
    let flags = [true, false];                 // [true] [false]
    while counter < flags.len() {              // [while]
        counter += 1;
    }

    let mut n: usize = 0;
    loop {                                     // [loop]
        n += 1;
        if n == 3 {
            continue;                          // [continue]
        }
        if n >= MAX_ITEMS {
            break;                             // [break]
        }
    }
    println!("loop finished at n = {}", n);

    trait_label!("T5 fearless concurrency: OS threads with move, then async/await");
    let handle = spawn_worker(vec![1, 2, 3, 4]);
    let sum = handle.join().unwrap();
    println!("thread sum = {}", sum);

    let greeting = fetch_greeting().await;     // [await]
    println!("{}", greeting);

    let ffi_value = unsafe_ffi_example().await;
    println!("unsafe-derived value = {}", ffi_value);
    println!("abs(-7) via extern C = {}", call_c_abs(-7));
}
