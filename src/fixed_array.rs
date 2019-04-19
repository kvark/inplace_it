/// This trait is a extended copy of unstable
/// [core::array::FixedSizeArray](core::array::FixedSizeArray).
///
/// This is not a perfect solution. Inheritance from `AsRef<[T]> + AsMut<[T]>` would be preferable.
/// But until we cannot implement `std` traits for `std` types so that inheritance limits us
/// and we cannot use `[T; n]` where `n > 32`.
pub trait FixedArray {
    type Item;
    fn len() -> usize;
    fn as_slice(&self) -> &[Self::Item];
    fn as_slice_mut(&mut self) -> &mut [Self::Item];
}

macro_rules! impl_fixed_array_for_array {
    ($($length: expr),+) => {
        $(
            impl<T> FixedArray for [T; $length] {
                type Item = T;
                #[inline]
                fn len() -> usize {
                    $length
                }
                #[inline]
                fn as_slice(&self) -> &[Self::Item] {
                    self
                }
                #[inline]
                fn as_slice_mut(&mut self) -> &mut [Self::Item] {
                    self
                }
            }
        )+
    };
}

macro_rules! impl_fixed_array_for_array_group_32 {
    ($($length: expr),+) => {
        $(
            impl_fixed_array_for_array!(
                $length, $length + 1, $length + 2, $length + 3,
                $length + 4, $length + 5, $length + 6, $length + 7,
                $length + 8, $length + 9, $length + 10, $length + 11,
                $length + 12, $length + 13, $length + 14, $length + 15,
                $length + 16, $length + 17, $length + 18, $length + 19,
                $length + 20, $length + 21, $length + 22, $length + 23,
                $length + 24, $length + 25, $length + 26, $length + 27,
                $length + 28, $length + 29, $length + 30, $length + 31
            );
        )+
    };
}

impl_fixed_array_for_array_group_32!(0, 32, 64, 96);

#[cfg(target_pointer_width = "16")]
impl_fixed_array_for_array!(
    160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480, 512, 544, 576, 608, 640, 672, 704, 736,
    768, 800, 832, 864, 896, 928, 960, 992, 1024, 1056, 1088, 1120, 1152, 1184, 1216, 1248, 1280,
    1312, 1344, 1376, 1408, 1440, 1472, 1504, 1536, 1568, 1600, 1632, 1664, 1696, 1728, 1760, 1792,
    1824, 1856, 1888, 1920, 1952, 1984, 2016, 2048, 2080, 2112, 2144, 2176, 2208, 2240, 2272, 2304,
    2336, 2368, 2400, 2432, 2464, 2496, 2528, 2560, 2592, 2624, 2656, 2688, 2720, 2752, 2784, 2816,
    2848, 2880, 2912, 2944, 2976, 3008, 3040, 3072, 3104, 3136, 3168, 3200, 3232, 3264, 3296, 3328,
    3360, 3392, 3424, 3456, 3488, 3520, 3552, 3584, 3616, 3648, 3680, 3712, 3744, 3776, 3808, 3840,
    3872, 3904, 3936, 3968, 4000, 4032, 4064, 4096
);

#[cfg(target_pointer_width = "32")]
impl_fixed_array_for_array!(
    160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480, 512, 544, 576, 608, 640, 672, 704, 736,
    768, 800, 832, 864, 896, 928, 960, 992, 1024, 1056, 1088, 1120, 1152, 1184, 1216, 1248, 1280,
    1312, 1344, 1376, 1408, 1440, 1472, 1504, 1536, 1568, 1600, 1632, 1664, 1696, 1728, 1760, 1792,
    1824, 1856, 1888, 1920, 1952, 1984, 2016, 2048, 2080, 2112, 2144, 2176, 2208, 2240, 2272, 2304,
    2336, 2368, 2400, 2432, 2464, 2496, 2528, 2560, 2592, 2624, 2656, 2688, 2720, 2752, 2784, 2816,
    2848, 2880, 2912, 2944, 2976, 3008, 3040, 3072, 3104, 3136, 3168, 3200, 3232, 3264, 3296, 3328,
    3360, 3392, 3424, 3456, 3488, 3520, 3552, 3584, 3616, 3648, 3680, 3712, 3744, 3776, 3808, 3840,
    3872, 3904, 3936, 3968, 4000, 4032, 4064, 4096
);

#[cfg(target_pointer_width = "64")]
impl_fixed_array_for_array!(
    160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480, 512, 544, 576, 608, 640, 672, 704, 736,
    768, 800, 832, 864, 896, 928, 960, 992, 1024, 1056, 1088, 1120, 1152, 1184, 1216, 1248, 1280,
    1312, 1344, 1376, 1408, 1440, 1472, 1504, 1536, 1568, 1600, 1632, 1664, 1696, 1728, 1760, 1792,
    1824, 1856, 1888, 1920, 1952, 1984, 2016, 2048, 2080, 2112, 2144, 2176, 2208, 2240, 2272, 2304,
    2336, 2368, 2400, 2432, 2464, 2496, 2528, 2560, 2592, 2624, 2656, 2688, 2720, 2752, 2784, 2816,
    2848, 2880, 2912, 2944, 2976, 3008, 3040, 3072, 3104, 3136, 3168, 3200, 3232, 3264, 3296, 3328,
    3360, 3392, 3424, 3456, 3488, 3520, 3552, 3584, 3616, 3648, 3680, 3712, 3744, 3776, 3808, 3840,
    3872, 3904, 3936, 3968, 4000, 4032, 4064, 4096
);

#[cfg(target_pointer_width = "128")]
impl_fixed_array_for_array!(
    160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480, 512, 544, 576, 608, 640, 672, 704, 736,
    768, 800, 832, 864, 896, 928, 960, 992, 1024, 1056, 1088, 1120, 1152, 1184, 1216, 1248, 1280,
    1312, 1344, 1376, 1408, 1440, 1472, 1504, 1536, 1568, 1600, 1632, 1664, 1696, 1728, 1760, 1792,
    1824, 1856, 1888, 1920, 1952, 1984, 2016, 2048, 2080, 2112, 2144, 2176, 2208, 2240, 2272, 2304,
    2336, 2368, 2400, 2432, 2464, 2496, 2528, 2560, 2592, 2624, 2656, 2688, 2720, 2752, 2784, 2816,
    2848, 2880, 2912, 2944, 2976, 3008, 3040, 3072, 3104, 3136, 3168, 3200, 3232, 3264, 3296, 3328,
    3360, 3392, 3424, 3456, 3488, 3520, 3552, 3584, 3616, 3648, 3680, 3712, 3744, 3776, 3808, 3840,
    3872, 3904, 3936, 3968, 4000, 4032, 4064, 4096
);
