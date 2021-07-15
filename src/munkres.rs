/**
 * Based of the following page: https://users.cs.duke.edu/~brd/Teaching/Bio/asmb/current/Handouts/munkres.html
 */
use log::{debug,info};

const MAX : i32 = 1000000;

const STAR : usize = 1;
const PRIME : usize = 2;
const NOT_FOUND : usize = 0;
const NODE_TYPES : usize = 3;

use prettytable::{Table, Row, Cell};
use std::fmt::Display;
use std::default::Default;
use std::collections::HashMap;

pub type Column<T> = Vec<T>;
pub type Matrix<T> = Vec<Vec<T>>;

struct State {
    path : Matrix<usize>,
    mask : Matrix<usize>,
    array : Matrix<i32>,
    row_cover : Column<bool>,
    column_cover : Column<bool>
}

impl State {
    pub fn new(array : Matrix<i32>) -> State {
        let size = array.len();
        State {
            array,
            path: matrix(size, NODE_TYPES),
            mask: square(size),
            row_cover: vec![false; size],
            column_cover: vec![false; size]
        }
    }
}

pub fn square<T: Default + Clone> (size : usize) -> Matrix<T> {
    matrix(size, size)
} 

pub fn matrix<T: Default + Clone> (rows : usize, cols : usize) -> Matrix<T> {
    vec![vec![T::default(); cols]; rows]
} 

fn row_min(array: &Matrix<i32> , row : usize) -> i32 {
    let size : usize = array.len();
    let mut min : i32 = MAX;
    for i in  1..size {
        if min > array[row][i] {
            min = array[row][i];
        }
    }
    min
}

fn subtract_row(array : &mut Matrix<i32>, row : usize, num : i32) {
    let size : usize = array.len();
    for i in 1..size {
        array[row][i] = array[row][i] - num;
    }
}

fn cover_column(column_cover : &mut Column<bool>, col : usize) {
    column_cover[col] = true;
}

fn cover(state : &mut State, row : usize, col : usize) {
    if state.array[row][col] == 0 && !state.row_cover[row] && !state.column_cover[col] {
        state.mask[row][col] = STAR;
        state.row_cover[row] = true;
        state.column_cover[col] = true;
    }
}

fn find_zero(array : &mut Matrix<i32>, row_cover : &mut Column<bool>, column_cover : &mut Column<bool>) -> (usize, usize) {
    let size = array.len();
    for i in 1..size {
        for j in 1..size {
            if array[i][j] == 0 && !row_cover[i] && !column_cover[j] {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn get_row_mask(mask : &mut Matrix<usize>, row : usize, val : usize) -> usize {
    let size = mask.len();
    for i in 1..size {
        if mask[row][i] == val {
            return i;
        }
    }
    NOT_FOUND
}

fn get_col_mask(mask : & Matrix<usize>, col : usize, val : usize) -> usize {
    let size = mask.len();
    for i in 1..size {
        if mask[i][col] == val {
            return i;
        }
    }
    NOT_FOUND
}

fn clear_covers(row_cover : &mut Column<bool>, column_cover : &mut Column<bool>) {
    let size = row_cover.len();
    for i in 1..size {
        row_cover[i] = false;
        column_cover[i] = false;
    }
}

fn clear_mask(mask : &mut Matrix<usize>, val : usize) {
    let size = mask.len();
    for i in 1..size {
        for j in 1..size {
            if mask[i][j] == val {
                mask[i][j] = 0;
            }
        }
    }
}

fn augment_path(path : &mut Matrix<usize>, mask : &mut Matrix<usize>, row : usize, col : usize) {
    let size = path.len();
    debug!("augment_path.");
    let mut count : usize = 1;
    path[count][STAR] = row;
    path[count][PRIME] = col;
    
    loop {
        // Find the row that contains a star'd zero using the column that contains a 
        // prime'd zero with *no* star'd zero.
        let row : usize = get_col_mask(mask, path[count][PRIME], STAR);
        //println!("augment_path_row: {}", row);
        if row > 0 {
             count = count + 1;
             path[count][STAR] = row;
             path[count][PRIME] = path[count - 1][PRIME];
        } else {
            break
        }
        // Find prime in row.
        let col : usize = get_row_mask(mask, path[count][STAR], PRIME);
        count = count + 1;
        path[count][STAR] = path[count - 1][STAR];
        path[count][PRIME] = col;
    }

    // Perform augmentation.
    for i in 1..size {
        let star : usize = path[i][STAR];
        let prime : usize = path[i][PRIME];
        if mask[star][prime] == STAR {
            mask[star][prime] = PRIME;
        } else {
            mask[star][prime] = STAR;
        }
    }
}

fn solved(column_cover : &Column<bool>) -> bool {
    let size = column_cover.len();
    for i in 1..size {
        if !column_cover[i] {
            return false;
        }
    }
    info!("solved!");
    
    true
}

// fn covered(row_cover : &Column<bool>, column_cover : &Column<bool>, row : usize, col : usize) -> bool {
//     row_cover[row] & column_cover[col]
// }

fn find_min(array : &mut Matrix<i32>, row_cover : & Column<bool>, column_cover : & Column<bool>) -> i32 {
    let size = array.len();
    let mut min : i32 = i32::MAX;
    for i in 1..size {
        for j in 1..size {
            if !column_cover[j] && !row_cover[i] {
                min = i32::min(min, array[i][j]);
            }
        }
    }
    min
}

// Returns the position of an prime'd zero that does not contain a star'd zero in the
// same row.
fn prime(state : &mut State,
    row : &mut usize,
    col : &mut usize) {
    debug!("prime.");

    loop {
        let (zero_row, zero_col) = find_zero(&mut state.array, &mut state.row_cover, &mut state.column_cover);
        if zero_row == 0 {
            *row = 0;
            *col = zero_col;
            return;
        }
        state.mask[zero_row][zero_col] = PRIME;

        let star_col = get_row_mask(&mut state.mask, zero_row, STAR);
        //println!("zero_row: {}, zero_col: {}, star_col: {}", zero_row, zero_col, star_col);
        if star_col != NOT_FOUND {
           state.row_cover[zero_row] = true;
           state.column_cover[star_col] = false;
        } else {
            *row = zero_row;
            *col = zero_col;
            return;
        }
    }
}

fn augment_weights(array : &mut Matrix<i32>,
    row_cover : & Column<bool>,
    column_cover : & Column<bool>) {
    debug!("augment_weights.");

    let size = array.len();
    let min : i32 = find_min(array, row_cover, column_cover);
    if min == i32::MAX {
        return;
    }
    for i in 1..size {
        for j in 1..size {
            if row_cover[i] {
                array[i][j] = array[i][j] + min;
            }
            if !column_cover[j] {
                array[i][j] = array[i][j] - min;
            }
        }
    }
}

pub fn solve(array : Matrix<i32>, keys : HashMap<usize, (String, usize)>) {

    let size = array.len();
    let mut state : State = State::new(array);
    // Keep previous state to ensure progress and help debuggability.
    let mut copy = State::new(square(size));
    
    // 1.
    for i in 1..size {
        let min = row_min(&state.array, i);
        subtract_row(&mut state.array, i, min);
    }
    // 2.
    for i in 1..size {
        for j in 1..size {
            cover(&mut state, i, j);
        }
    }
    clear_covers(&mut state.row_cover, &mut state.column_cover);

    // Decides whether the next iteration of the loop is to start at step #3 or step #4.
    let mut offset = false;
    loop {
        // 3.
        if !offset {
            for i in 1..size {
                for j in 1..size {
                    if state.mask[i][j] == STAR {
                        cover_column(&mut state.column_cover, j);
                    }
                }
            }
            if solved(&state.column_cover) {
                print_matrix(state.mask, keys);
                return
            }
            offset = true
        }
        let (mut row, mut col) = (0, 0);
        // 4.
        if offset {
            prime(&mut state, &mut row, &mut col);
        }
        // 5.
        if row > 0 && col > 0 {
            // A path starting with a prime'd zero was found, so try to augment the PATH to increase the
            // size of the matching.
            augment_path(&mut state.path, &mut state.mask, row, col);
            clear_covers(&mut state.row_cover, &mut state.column_cover);
            clear_mask(&mut state.mask, PRIME);
            offset = false;
        } else {
            // 6.
            augment_weights(&mut state.array, &state.row_cover, &state.column_cover);
        }

        if solved(&state.column_cover) {
            print_matrix(state.mask, keys);
            return;
        }

        if !makes_progress(&state, &mut copy) {
            info!("col_covered_count: {}, row_covered_count: {}", col_covered_count(&mut state.column_cover), row_covered_count(&mut state.column_cover));
            return;
        }
    }

}

fn col_covered_count(column_cover : &mut Column<bool>) -> i32 {
    let mut count : i32 = 0;
    for i in 1..column_cover.len() {
        if column_cover[i] {
            count = count + 1;
        }
    }
    count
}

fn row_covered_count(row_cover : &mut Column<bool>) -> i32 {
    let mut count : i32 = 0;
    for i in 1..row_cover.len() {
        if row_cover[i] {
            count = count + 1;
        }
    }
    count
}

fn print_matrix<T: Display>(matrix : Matrix<T>, keys : HashMap<usize, (String, usize)>) {
    let mut table = Table::new();
    
    let mut header : Vec<Cell> = vec![Cell::new(" "), Cell::new(" ")];

    let size = matrix.len();

    for i in 1..size {
        header.push(Cell::new(&i.to_string()));
    }
    table.add_row(Row::new(header));
    
    for i in 1..size {
        let mut row : Vec<Cell> = Vec::new();
        //let (name, key) = keys.get(&i).unwrap();
        match keys.get(&i) {
            Some((name, _)) => {
                row.push(Cell::new(&name).style_spec("bFr"));
            },
            None => {
                panic!("Not found: {}", &i);
            }
        }
        row.push(Cell::new(&format!("{}", &i.to_string())));
        for j in 1..size {
            let style : String = String::new();
            if &matrix[i][j] == 1 {
                style.push_str("bFg");
            }
            row.push(Cell::new(&matrix[i][j].to_string()).style_spec(&style));
        }
        table.add_row(Row::new(row));
    }
    table.printstd();
}

fn makes_progress(state : & State, old : &mut State) -> bool {
   
    let size : usize = state.array.len();
    let mut progress : bool = false;
    for i in 1..size {
        for j in 1..size {
            if old.array[i][j] != state.array[i][j] {
                progress = true;
            }
            old.array[i][j] = state.array[i][j];
        }
    }
    for i in 1..size {
        for j in 1..size {
            if old.mask[i][j] != state.mask[i][j] {
                progress = true;
            }
            old.mask[i][j] = state.mask[i][j];
        }
    }

    for i in 1..size {
        if old.column_cover[i] != state.column_cover[i] {
            progress = true;
        }
        old.column_cover[i] = state.column_cover[i];
    }
    for i in 1..size {
        if old.row_cover[i] != state.row_cover[i] {
            progress = true;
        }
        old.row_cover[i] = state.row_cover[i];
    }

    for i in 1..size {
        for j in 1..NODE_TYPES {
            if old.path[i][j] != state.path[i][j] {
                progress = true;
            }
            old.path[i][j] = state.path[i][j];
        }
    }

    return progress;
}