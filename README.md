## Vector Functions

---

This library implements various functions on 2D vectors (matrices in
linear algebra).
So far, only addition and subtraction are implemented, but eventually
multiplication, the dot product, cross product, and more will be 
implemented.

# Example code
```rust
extern crate test_vec;
use test_vec::vec_func;

fn main() {
    let mut h = Vec::new();
    h.push(vec![1,2,3,4]);
    h.push(vec![2,3,4,5]);
    let mut v = Vec::new();
    v.push(vec![1,2,3,4]);
    v.push(vec![2,3,4,5]);

    println!("Adding: {:?}", vec_func::add(h,v));

    let mut h = Vec::new();
    h.push(vec![1,2,3,4]);
    h.push(vec![2,3,4,5]);
    let mut v = Vec::new();
    v.push(vec![1,2,3,4]);
    v.push(vec![2,3,4,5]);


    println!("Subtracting: {:?}", vec_func::subtract(h,v));
}
```
