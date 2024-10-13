use std::mem;

fn main() {
    // You can optionally experiment here.
    let a: [i8; 5] = [1, 2, 3, 4, 5];

    println!("Size of array in memory {} bytes", mem::size_of_val(&a));
}

#[cfg(test)]
mod tests {

    use std::mem;

    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        println!("Size of array in memory {}", mem::size_of_val(&a));

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);
    }
}