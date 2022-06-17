
// as best as I can figure, this is whats happening in this example
//
// the function declaration is saying that the foo function takes parameter a as an i32 value and f
// as a function definited by F which is a function that takes a i32 value and returns an i32 value
// then foo returns an i32 value
fn foo<F: Fn(i32) -> i32>(a: i32, f: F) -> i32 {
    f(a)
}

fn main() {
    let bar = foo(5, |x| { x + 1 });
    println!("{}", bar);
}
