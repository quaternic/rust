#![feature(intrinsics)]

// Directly call intrinsic to avoid debug assertions in libstd
#[rustc_intrinsic]
unsafe fn float_to_int_unchecked<Float: Copy, Int: Copy>(value: Float) -> Int;

fn main() {
    unsafe {
        float_to_int_unchecked::<f64, i128>(-240282366920938463463374607431768211455.0f64); //~ ERROR: cannot be represented in target type `i128`
    }
}
