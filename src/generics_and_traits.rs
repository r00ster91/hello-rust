// this is nice reuseable code
fn largest(ints: &[i32]) -> &i32 {
    let mut largest = &ints[0];
    for int in ints {
        if int > largest {
            largest = int;
        }
    }
    largest
}

// this will only compile for types that implement the PartialOrd (>) trait!
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//Traits and trait bounds let us write code that uses generic type parameters
// to reduce duplication but also specify to the compiler that we want the generic
// type to have particular behavior

// this will only compile for types that implement both the PartialOrd (>) and the Copy ([0]) trait!
// these conditions are called trait bounds
fn largest3<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn main() {
    println!("{}", largest(&vec![1,2, 3,232,3,23,23,2,32,32,3,23,392992]));
    println!("{}", largest(&vec![55,2, 3,232,3,23,555555,2,32,555,3,23,392992]));

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, &100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, &'y');

    let chars = Point { x: 'a', y: 5 };
    let float = Point { x: 1.0, y: 4.0 };

    let p = Pos { x: 5, y: 10 };

    println!("p.x = {}", p.x());

}

struct Pos<T> {
    x: T,
    y: T,
}

impl<T> Pos<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// and this only defines for a specific type
impl Pos<f32> {
    // Calculates the distance of the position from (0.0, 0.0)
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


// this is called a GENERIC TYPE PARAMETER
// we can name these generic type parameters whatever the hell we want!
// not just T, U
struct Point<Wa, Xwe> {
    x: Wa,
    y: Xwe,
}

fn a(list: &i32) {
}

fn b(list: &i8) {}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

