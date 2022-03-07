// SCALAR TYPES
/* 
- signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
- unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
- floating point: f32, f64
- char Unicode scalr values like a (4 bytes each)
- bool either true or false
- and unit type (), whose only possible value is an empty tuple: ()
*/

// COMPOUND TYPES
/*
- arrays like [1, 2, 3]
- tuples like (1, true)
*/

fn main() {
    // VAriables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used. 
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 39i64;

    // A mutable variables value can be changed
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // ERROR! The type of variable cannot be changed
    // mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true;

}
