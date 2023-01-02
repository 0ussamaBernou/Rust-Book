use std::mem;
fn main() {
    let t = (1, (2, false));
    // pretty print
    println!("{:#?}", t);
    // initializing array of fixed size(5) of type :i32
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // length of array 'xs'
    let arr_length = xs.len();
    // size of array 'xs' in bytes
    let arr_size = mem::size_of_val(&xs);
    println!("{:?} {} {} ", xs, arr_length, arr_size);

    // function that doesn't have a return statement return an empty tuple -> ()
}
