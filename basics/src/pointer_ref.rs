// Reference Pointers - Point to a resource in memory
// => BORROWING

#[derive(Clone,Copy)]
struct Value {}

pub fn run()
{
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1,arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first variable will NO LONGER hold that value (!!).
    // You'll need to use a reference (&) to point to the resource.

    // Vector
    let vec1 : Vec<i32> = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1,vec2));
}

/* 
A value was moved out while it was still borrowed.

Erroneous code example:

```
    struct Value {}

    fn eat(val: Value) {}

    fn main() {
        let x = Value{};
        {
            let _ref_to_val: &Value = &x;
            eat(x);
        }
    }
```

Here, the function `eat` takes the ownership of `x`. However,
`x` cannot be moved because it was borrowed to `_ref_to_val`.
To fix that you can do few different things:

* Try to avoid moving the variable.
* Release borrow before move.
* Implement the `Copy` trait on the type.

Examples:

```
struct Value {}

fn eat(val: &Value) {}

fn main() {
    let x = Value{};
    {
        let _ref_to_val: &Value = &x;
        eat(&x); // pass by reference, if it's possible
    }
}
```

Or:

```
struct Value {}

fn eat(val: Value) {}

fn main() {
    let x = Value{};
    {
        let _ref_to_val: &Value = &x;
    }
    eat(x); // release borrow and then move it.
}
```

Or:

```
#[derive(Clone, Copy)] // implement Copy trait
struct Value {}

fn eat(val: Value) {}

fn main() {
    let x = Value{};
    {
        let _ref_to_val: &Value = &x;
        eat(x); // it will be copied here.
    }
}
```

You can find more information about borrowing in the rust-book:
http://doc.rust-lang.org/stable/book/references-and-borrowing.html

*/