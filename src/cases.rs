const SIZE : usize = 1000;

fn write_row(array : &mut [[i32; SIZE]; SIZE], row : Vec<i32>, pos : usize) {
    for i in 0..row.len() {
        array[pos][i+1] = row[i];
    }
}

pub fn simple_diagonal(array : &mut [[i32; SIZE]; SIZE]) {
    write_row(array, vec![1, 2, 3], 1);
    write_row(array, vec![2, 4, 6], 2);
    write_row(array, vec![3, 6, 9], 3);
}

pub fn simple_random(array : &mut [[i32; SIZE]; SIZE]) {
    write_row(array, vec![10, 2, 10], 1);
    write_row(array, vec![10, 10, 6], 2);
    write_row(array, vec![3, 10, 10], 3);
}