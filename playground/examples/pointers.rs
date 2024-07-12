fn main() {
    let a = 42;
    let memory_location = &a as *const i32 as usize;
    println!("p: {memory_location:0x}");
    println!("p: {:p}", &a);
}
