use std::{
    arch::asm,
    slice::{from_raw_parts, from_raw_parts_mut},
    str::from_utf8_unchecked,
};

fn get_memory_location() -> (usize, usize) {
    let string = "Hello, world!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

unsafe fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            from_raw_parts_mut(ptr, mid),
            from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn test1() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
    }

    let address = 0x012345usize;
    let r = address as *const i32;

    let (pointer, length) = get_memory_location();

    unsafe {
        let message = get_str_at_location(pointer, length);

        println!(
            "The {} bytes at 0x{:X} stored: {}",
            length, pointer, message
        );
    }

    let a: Box<i32> = Box::new(10);
    // 需要先解引用a
    let b: *const i32 = &*a;
    // 使用 into_raw 来创建
    let c: *const i32 = Box::into_raw(a);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn test() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    let x: u64;
    unsafe {
        asm!("mov {}, 5", out(reg) x);
    }
    assert_eq!(x, 5);

    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );
    }
    assert_eq!(o, 8);

    let mut x: u64 = 3;
    unsafe {
        asm!("add {0}, 5", inout(reg) x);
    }
    assert_eq!(x, 8);
}
