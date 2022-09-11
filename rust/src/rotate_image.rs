/// If we want to rotate a matrix

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    // 11 12 21 22 
    // 13 14 23 24
    // 31 32 41 42
    // 33 34 43 44
   
    // We could potentially do the swaps of `1 <-> 2` and `3 <-> 4`at the same time and they would 
    // move `1` and `3` into their correct segments respectively.

    // 1 <-> 2
    // 21 22 11 12 
    // 23 24 13 14 
    // 31 32 41 42
    // 33 34 43 44

    // 3 <-> 4
    // 21 22 11 12 
    // 23 24 13 14 
    // 41 42 31 32 
    // 43 44 33 34 

    // Barrier
    // 2 <-> 4
    // 41 42 11 12 
    // 43 44 13 14 
    // 21 22 31 32 
    // 23 24 33 34 

    inner_rotate(matrix,(0..matrix.len(),0..matrix.len()));
}
fn inner_rotate(matrix: &mut Vec<Vec<i32>>, (y,x): (std::ops::Range<usize>,std::ops::Range<usize>)) {
    println!("inner_rotate: {:?}",(&y,&x));
    let l = x.end - x.start;
    debug_assert_eq!(x.end - x.start, y.end - y.start);
    let n = l as f32 / 2f32;
    let n_floor = n.floor() as usize;
    let mx = x.start as f32 + n;
    let my = y.start as f32 + n;
    

    // 1 <-> 2
    let mx_floor = mx.floor() as usize;
    let mx_ceil = mx.ceil() as usize;
    for i in y.start..n_floor {
        println!("{:?} <-> {:?}",&matrix[i][x.start..mx_floor],&matrix[i][mx_ceil..(mx_ceil+n_floor)]);
        unsafe {
            std::ptr::swap_nonoverlapping((&matrix[i][x.start]) as *const i32 as *mut i32,(&matrix[i][mx_ceil])as *const i32 as *mut i32,n_floor);
        }
    }
    print_matrix(matrix);

    // 3 <-> 4
    let my_floor = my.floor() as usize;
    let my_ceil = my.ceil() as usize;
    for i in my_ceil..y.end {
        println!("{:?} <-> {:?}",&matrix[i][x.start..mx_floor],&matrix[i][mx_ceil..(mx_ceil+n_floor)]);
        unsafe {
            std::ptr::swap_nonoverlapping((&matrix[i][x.start]) as *const i32 as *mut i32,(&matrix[i][mx_ceil])as *const i32 as *mut i32,n_floor);
        }
    }
    print_matrix(matrix);

    // Barrier
    // 2 <-> 4
    for (i,j) in (y.start..my_floor).zip(my_ceil..y.end) {
        println!("{:?} <-> {:?}",&matrix[i][x.start..(x.start+n_floor)],&matrix[j][mx_ceil..(mx_ceil+n_floor)]);
        unsafe {
            std::ptr::swap_nonoverlapping((&matrix[i][x.start]) as *const i32 as *mut i32,(&matrix[j][mx_ceil]) as *const i32 as *mut i32,n_floor);
        }
    }
    print_matrix(matrix);

    if n_floor > 1 {
        inner_rotate(matrix,(y.start..my_floor,x.start..mx_floor));
        inner_rotate(matrix,(y.start..my_floor,mx_ceil..x.end));
        inner_rotate(matrix,(my_ceil..y.end,x.start..mx_floor));
        inner_rotate(matrix,(my_ceil..y.end,mx_ceil..x.end));
    }
}
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{} ",matrix[i][j])
        }
        println!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotate_image_simple() {
        let mut matrix = vec![
            vec![10,11,12,20,21,22],
            vec![13,14,15,23,24,25],
            vec![16,17,18,26,27,28],
            vec![40,41,42,31,32,33],
            vec![43,44,45,34,35,36],
            vec![46,47,48,37,38,39],
        ];
        rotate(&mut matrix);
        let mut expected = vec![
            vec![46,43,40,16,13,10],
            vec![47,44,41,17,14,11],
            vec![48,45,42,18,15,12],
            vec![37,34,31,26,23,20],
            vec![38,35,32,27,24,21],
            vec![39,36,33,28,25,22],
        ];
    }
}