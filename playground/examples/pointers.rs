use std::error::Error;

fn _basic() {
    let a = 42;
    let memory_location = &a as *const i32 as usize;
    println!("p: {memory_location:0x}");
    println!("p: {:p}", &a);
}

fn _row_pointer_over_vec() {
    let v = vec![1, 2, 3];
    let base_pointer = &v as *const Vec<i32> as usize;
    for offset in -8..8i32 {
        let p = base_pointer as i32 + 4 * offset;
        let p = p as *const i32;
        let pv = unsafe { *p };
        println!("pv: {pv:?}");
    }
}

fn main() {
    _row_pointer_over_vec();
}
