pub fn main() -> () {
    structures();
    enums();
    enums_linked_lists();
    constants();
}

fn structures() -> () {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // A unit struct
    #[derive(Debug)]
    struct Unit;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    #[derive(Debug)]
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }
    impl Rectangle {
        fn area(&self) -> f32 {
            let Rectangle {
                top_left:
                    Point {
                        x: top_left_x,
                        y: top_left_y,
                    },
                bottom_right:
                    Point {
                        x: bottom_right_x,
                        y: bottom_right_y,
                    },
            } = self;
            let width = bottom_right_x - top_left_x;
            let height = top_left_y - bottom_right_y;

            width * height
        }
        fn square(point: Point, size: f32) -> Rectangle {
            Rectangle {
                top_left: Point {
                    x: point.x,
                    y: point.y + size,
                },
                bottom_right: Point {
                    x: point.x + size,
                    y: point.y,
                },
            }
        }
    }

    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    println!("({}, {})", top_edge, left_edge);

    // Instantiate a unit struct
    let unit = Unit;
    println!("{:?}", unit);

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle {
        top_left: Point { x: 1.0, y: 10.4 },
        bottom_right: Point { x: 10.3, y: 0.4 },
    };
    println!(
        "Area for rect {rect:?} is: {area}",
        rect = rect,
        area = rect.area()
    );

    println!(
        "New Square rect {rect:?}",
        rect = Rectangle::square(Point { x: 3.0, y: 3.0 }, 2.0),
    );
}

fn enums() -> () {
    // Create an `enum` to classify a web event. Note how both
    // names and type information together specify the variant:
    // `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
    // Each is different and independent.
    enum WebEvent {
        // An `enum` may either be `unit-like`,
        PageLoad,
        PageUnload,
        // like tuple structs,
        KeyPress(char),
        Paste(String),
        // or c-like structures.
        Click { x: i64, y: i64 },
    }

    // A function which takes a `WebEvent` enum as an argument and
    // returns nothing.
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum`.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum Status {
        #[allow(dead_code)]
        Rich,
        Poor,
    }
    enum Work {
        Civilian,
        #[allow(dead_code)]
        Soldier,
    }

    // Explicitly `use` each name so they are available without
    // manual scoping.
    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // enum with implicit discriminator (starts at 0)
    enum Number {
        Zero,
        One,
        #[allow(dead_code)]
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        #[allow(dead_code)]
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

fn enums_linked_lists() -> () {
    enum List {
        // Cons: Tuple struct that wraps an element and a pointer to the next node
        Cons(u32, Box<List>),
        // Nil: A node that signifies the end of the linked list
        Nil,
    }
    use List::*;
    impl List {
        // Create an empty list
        fn new() -> List {
            // `Nil` has type `List`
            Nil
        }

        // Consume a list, and return the same list with a new element at its front
        fn prepend(self, elem: u32) -> List {
            // `Cons` also has type List
            Cons(elem, Box::new(self))
        }

        // Consume a list, and return the same list with a new element at its end
        fn append(self, element: u32) -> List {
            match self {
                Cons(e, tail) => Cons(e, Box::new(tail.append(element))),
                Nil => Cons(element, Box::new(Nil)),
            }
        }

        // Return the length of the list
        fn len(&self) -> u32 {
            // `self` has to be matched, because the behavior of this method
            // depends on the variant of `self`
            // `self` has type `&List`, and `*self` has type `List`, matching on a
            // concrete type `T` is preferred over a match on a reference `&T`
            match self {
                // Can't take ownership of the tail, because `self` is borrowed;
                // instead take a reference to the tail
                Cons(_, ref tail) => 1 + tail.len(),
                // Base Case: An empty list has zero length
                Nil => 0,
            }
        }

        // Return representation of the list as a (heap allocated) string
        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    // `format!` is similar to `print!`, but returns a heap
                    // allocated string instead of printing to the console
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => {
                    format!("Nil")
                }
            }
        }
    }

    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    // Create an empty linked list
    let mut list = List::new();

    // Append some elements
    list = list.append(1);
    list = list.append(2);
    list = list.append(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn constants() {
    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }

    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line

    // Error! Cannot assign to immutable static item
    // LANGUAGE = "TEST";
}
