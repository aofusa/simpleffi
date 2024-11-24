use std::alloc::{alloc, dealloc, Layout};

#[no_mangle]
pub extern "C" fn simple() -> i32 { 42 }

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }

#[no_mangle]
pub extern "C" fn array_add(ptr: *mut i32, size: usize) {
    for i in 0..size as usize {
        unsafe { *ptr.add(i) += 1; }
    }
}

#[repr(C)]
pub struct Point {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn struct_add(p: *mut Point) {
    unsafe {
        (*p).x += 1;
        (*p).y += 1;
    }
}

#[no_mangle]
pub extern "C" fn memalloc(size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(size).expect("overflow");
    unsafe { alloc(layout).cast::<u8>() }
}

#[no_mangle]
pub extern "C" fn memfree(ptr: *mut u8, size: usize) {
    let layout = Layout::array::<u8>(size).expect("overflow");
    unsafe { dealloc(ptr, layout) }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use super::*;

    #[test]
    fn test_simple() {
        let result = simple();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_array() {
        let array = &mut [1, 2, 3];
        array_add(array.as_mut_ptr(), array.len());
        assert_eq!(array[0], 2);
        assert_eq!(array[1], 3);
        assert_eq!(array[2], 4);
    }

    #[test]
    fn test_struct() {
        let mut data = Point { x: 0, y: 1 };
        let p = &mut data as *mut Point;
        struct_add(p);
        assert_eq!(data.x, 1);
        assert_eq!(data.y, 2);
    }

    #[test]
    fn test_mem() {
        let ptr = memalloc(4 * 3).cast::<i32>();

        let v = unsafe {
            ptr.add(0).write(1);
            ptr.add(1).write(2);
            ptr.add(2).write(3);

            Vec::from_raw_parts(ptr, 3, 3)
        };

        assert_eq!(v, vec![1, 2, 3]);

        mem::forget(v);
        memfree(ptr.cast::<u8>(), 4 * 3);
    }
}
