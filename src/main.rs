fn main() {


    let msg = add_strs(String::from("Hello, "), String::from("World!"));
    dbg!(msg);

    let sum = add_nums(1, 1);
    dbg!(sum);

    let another_sum = add_generic(2, 2);
    dbg!(another_sum);

    let p1 = Point {x: 10, y: 10};
    let p2 = Point {x: 10, y: 10};
    let p3 = add_generic(p1, p2);
    dbg!(p3);

}

// a function which adds two strings
fn add_strs(s1: String, s2: String) -> String {
    s1 + &s2
}

// a function which adds two numbers
fn add_nums(x: i32, y: i32) -> i32 {
    x + y
}

// a generic function which can handle both String and i32 (along with other types
fn add_generic<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

// a custom type we might want to add
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// implementing add on our custom type
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

