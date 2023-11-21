use crate::{from_slice_iter::FromSliceIter, RangeSetBlaze};

use core::ops::RangeInclusive;
use core::simd::{i16x32, i32x16, i64x8, i8x64, LaneCount, Simd, SupportedLaneCount};
use std::mem::size_of;

// cmk Rule may want to skip sse2 (128) because it is slower than the non-simd version

use crate::Integer;

macro_rules! from_slice_etc {
    ($expected:ident) => {
        // avx512 (512 bits) or scalar
        #[cfg(any(target_feature = "avx512f", not(target_feature = "avx2")))]
        // cmk Rule: Making this inline reduced time from 146 to 92
        fn from_slice(slice: &[Self]) -> RangeSetBlaze<Self> {
            // cmk Rule: const expressions are handy.
            // Note: Does the right thing for isize, usize
            // cmk5 Look for other uses of const expressions
            FromSliceIter::<Self, { 64 / size_of::<Self>() }>::new(slice, unsafe {
                std::mem::transmute(&$expected)
            })
            .collect()
        }
        // avx2 (256 bits)
        #[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
        fn from_slice(slice: &[Self]) -> RangeSetBlaze<Self> {
            FromSliceIter::<Self, { 64 / size_of::<Self>() }>::new(slice).collect()
        }
    };
}

impl Integer for i8 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = usize;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = usize;

    from_slice_etc!(EXPECTED_X8);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as u8 as <Self as Integer>::SafeLen + 1
    }
    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for u8 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = usize;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = usize;

    from_slice_etc!(EXPECTED_X8);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as <Self as Integer>::SafeLen + 1
    }

    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

// avx512 (512 bits) or scalar
#[cfg(any(target_feature = "avx512f", not(target_feature = "avx2")))]
const EXPECTED_X8: i8x64 = unsafe {
    std::mem::transmute([
        -63i8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1,
    ])
};

// avx2 (256 bits)
#[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
const EXPECTED_X8: i8x32 = unsafe {
    std::mem::transmute([
        -31i8, 11, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1,
    ])
};

// avx512 (512 bits) or scalar
#[cfg(any(target_feature = "avx512f", not(target_feature = "avx2")))]
const EXPECTED_X16: i16x32 = unsafe {
    std::mem::transmute([
        -31i16, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1,
    ])
};
// avx2 (256 bits)
#[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
const EXPECTED_X16: i16x16 =
    unsafe { std::mem::transmute([-15i16, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]) };

// avx512 (512 bits) or scalar
#[cfg(any(target_feature = "avx512f", not(target_feature = "avx2")))]
const EXPECTED_X32: i32x16 =
    unsafe { std::mem::transmute([-15i32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]) };

// avx2 (256 bits)
#[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
const EXPECTED_X32: i32x8 = unsafe { std::mem::transmute([-7i32, 1, 1, 1, 1, 1, 1, 1]) };

// avx512 (512 bits) or scalar
#[cfg(any(target_feature = "avx512f", not(target_feature = "avx2")))]
const EXPECTED_X64: i64x8 = unsafe { std::mem::transmute([-7i64, 1, 1, 1, 1, 1, 1, 1]) };

// avx2 (256 bits)
#[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
const EXPECTED_X64: i64x4 = unsafe { std::mem::transmute([-3i64, 1, 1, 1]) };

impl Integer for i32 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = u64;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = usize;

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as u32 as <Self as Integer>::SafeLen + 1
    }

    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }

    from_slice_etc!(EXPECTED_X32);
}

impl Integer for u32 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = u64;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = usize;

    from_slice_etc!(EXPECTED_X32);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as <Self as Integer>::SafeLen + 1
    }

    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for i64 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = u128;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = u128;

    from_slice_etc!(EXPECTED_X64);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as u64 as <Self as Integer>::SafeLen + 1
    }
    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for u64 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = u128;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = u128;

    from_slice_etc!(EXPECTED_X64);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as <Self as Integer>::SafeLen + 1
    }
    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for i128 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = u128;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = u128;

    #[inline]
    fn from_slice(slice: &[Self]) -> crate::RangeSetBlaze<Self> {
        return slice.iter().collect();
    }

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as u128 as <Self as Integer>::SafeLen + 1
    }
    fn safe_max_value() -> Self {
        Self::max_value() - 1
    }

    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for u128 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = u128;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = u128;

    #[inline]
    fn from_slice(slice: &[Self]) -> crate::RangeSetBlaze<Self> {
        return slice.iter().collect();
    }

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as <Self as Integer>::SafeLen + 1
    }
    fn safe_max_value() -> Self {
        Self::max_value() - 1
    }
    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for isize {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = u64;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = u128;

    #[cfg(target_pointer_width = "32")]
    from_slice_etc!(EXPECTED_X32);
    #[cfg(target_pointer_width = "64")]
    from_slice_etc!(EXPECTED_X64);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as usize as <Self as Integer>::SafeLen + 1
    }

    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for usize {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = u64;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = u128;

    #[cfg(target_pointer_width = "32")]
    from_slice_etc!(EXPECTED_X32);
    #[cfg(target_pointer_width = "64")]
    from_slice_etc!(EXPECTED_X64);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as <Self as Integer>::SafeLen + 1
    }

    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for i16 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = usize;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = usize;

    from_slice_etc!(EXPECTED_X16);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as u16 as <Self as Integer>::SafeLen + 1
    }

    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

impl Integer for u16 {
    #[cfg(target_pointer_width = "32")]
    type SafeLen = usize;
    #[cfg(target_pointer_width = "64")]
    type SafeLen = usize;

    from_slice_etc!(EXPECTED_X16);

    fn safe_len(r: &RangeInclusive<Self>) -> <Self as Integer>::SafeLen {
        r.end().overflowing_sub(*r.start()).0 as <Self as Integer>::SafeLen + 1
    }

    fn safe_len_to_f64(len: Self::SafeLen) -> f64 {
        len as f64
    }
    fn f64_to_safe_len(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }
    fn add_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a + (b - 1) as Self
    }
    fn sub_len_less_one(a: Self, b: Self::SafeLen) -> Self {
        a - (b - 1) as Self
    }
}

// cmk Rule: Look at the docs in a way that lets you see every useful command (how?)
// cmk Rule: You have to use nightly, so not usefull. (how to turn on for just one project)
// cmk Rule: As soon as you think about SIMD algorithms, you'll likely make non-faster
// cmk Rule: Set up for multiple levels of support
// cmk Rule: AMD 512 might be slower than Intel (but maybe not for permutations)
// cmk Rule: Docs: https://doc.rust-lang.org/nightly/std/simd/index.html
// cmk Rule: Docs: more https://doc.rust-lang.org/nightly/std/simd/struct.Simd.html
// cmk5 Tighter clippy, etc.
// cmk Rule: Expect operations to wrap. Unlike scalar it is the default.
// cmk Rule: Use #[inline] on functions that take a SIMD input and return a SIMD output (see docs)
// cmk Rule: It's generally OK to use the read "unaligned" on aligned. There is no penalty. (see https://doc.rust-lang.org/std/simd/struct.Simd.html#safe-simd-with-unsafe-rust)
// cmk Rule: Useful: https://github.com/rust-lang/portable-simd/blob/master/beginners-guide.md (talks about reduce_and, etc)
// cmk Rule: Do const values like ... https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/const.20SIMD.20values
// cmk Rule: Use SIMD rust command even without SIMD.
// cmk Rule: Use unsafe where you need to.

#[allow(unused_macros)]
macro_rules! check_simd {
    ($simd:expr) => {{
        let length = $simd.lanes();
        let t_bytes = std::mem::size_of_val(&$simd) / length;
        assert_eq!(length, 64 / t_bytes);
        assert_eq!($simd[0] as i32, -((length as i32) - 1));
        for &val in $simd.as_array().iter().skip(1) {
            assert_eq!(val, 1);
        }
    }};
}

// cmk Rule: Test your constants
#[test]
fn check_simd_constants() {
    check_simd!(EXPECTED_X8);
    check_simd!(EXPECTED_X16);
    check_simd!(EXPECTED_X32);
    check_simd!(EXPECTED_X64);
}
