// This is a way to add `Display` the struct and allow us to print the value
// This would be a macro
#[derive(Debug)]
struct Position {
    x: i8,
    y: i8,
}

impl Position {
    fn update_y(&mut self) {
        self.y += 1;
    }

    fn update_x(&mut self) {
        self.x -= 1;
        self.update_y();
    }
}

fn first() {
    // certain types that implement copy traits (copied instead of moved)
    let x = 3;
    let y = &x;
    println!("Hello, world! {}", y);
}

fn second() {
    let pos = Position { x: 3, y: 2 };
    // {:?} is to print debug (from derive debug)
    // {:#?} is pretty print of debug
    println!("My position is {:?}", pos);
    println!("My position is {:#?}", pos);
}

fn update_y(pos: &mut Position) {
    pos.y += 1;
}

fn update_x(pos: &mut Position) {
    pos.x -= 1;
    update_y(pos);
}

fn third() {
    let mut pos = Position { x: 3, y: 2 };
    update_x(&mut pos);
    println!("My position is {:?}", pos);
}

fn fourth() {
    let mut pos = Position { x: 3, y: 2 };
    pos.x = 13;
    // can call it on the object
    pos.update_x();
    // or we can call it directly :)
    Position::update_x(&mut pos);
    println!("My position is {:#?}", pos)
}

#[test]
fn funny_name() {
    assert_eq!(1, 1);
}

fn main() {
    first();
    second();
    third();
    fourth();
}
