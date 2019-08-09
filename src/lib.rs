
/// A generic interface for casting between machine scalars with the
/// `as` operator, which admits narrowing and precision loss.
/// Implementers of this trait PrimitiveFrom should behave like a primitive
/// numeric type (e.g. a newtype around another primitive), and the
/// intended conversion must never fail.
///
/// # Examples
///
/// ```
/// # use primitive_from::PrimitiveFrom;
/// let three: i32 = PrimitiveFrom::from(3.14159265f32);
/// assert_eq!(three, 3);
/// ```
/// 
/// # Safety
/// 
/// Currently, some uses of the `as` operator are not entirely safe.
/// In particular, it is undefined behavior if:
/// 
/// - A truncated floating point value cannot fit in the target integer
///   type ([#10184](https://github.com/rust-lang/rust/issues/10184));
/// 
/// ```ignore
/// # use primitive_from::PrimitiveFrom;
/// let x: u8 = PrimitiveFrom::from(1.04E+17); // UB
/// ```
/// 
/// - Or a floating point value does not fit in another floating
///   point type ([#15536](https://github.com/rust-lang/rust/issues/15536)).
///
/// ```ignore
/// # use primitive_from::PrimitiveFrom;
/// let x: f32 = PrimitiveFrom::from(1e300f64); // UB
/// ```
/// 
pub trait PrimitiveFrom<T>: 'static + Copy 
where
    T: 'static + Copy, 
{
    fn from(_:T) -> Self;
}

macro_rules! impl_primitive_from {
    ($U: ty => $( $T: ty ),* ) => {
        $(
        impl PrimitiveFrom<$U> for $T {
            #[inline] fn from(a:$U) -> $T { a as $T }
        }
        )*
    };
}

impl_primitive_from!(u8 => char, u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(i8 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(u16 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(i16 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(u32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(i32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(u64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(i64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(usize => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(isize => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(f32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(f64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_primitive_from!(char => char, u8, i8, u16, i16, u32, i32, u64, isize, usize, i64);
impl_primitive_from!(bool => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64);


#[test]
fn as_primitive() {
    let x: f32 = PrimitiveFrom::from(1.625f64);
    assert_eq!(x, 1.625f32);

    let x: f32 = PrimitiveFrom::from(3.14159265358979323846f64);
    assert_eq!(x, 3.1415927f32);

    let x: u8 = PrimitiveFrom::from(768i16);
    assert_eq!(x, 0);
}
