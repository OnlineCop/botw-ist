use std::sync::Arc;

/// Make a `gdt::FlagDescriptor` from shorthand
#[macro_export]
#[rustfmt::skip]
macro_rules! fd {
    (bool) => { blueflame::game::gdt::FdBool };
    (s32) => { blueflame::game::gdt::FdS32 };
    (f32) => { blueflame::game::gdt::FdF32 };
    (str32) => { blueflame::game::gdt::FdString32 };
    (str64) => { blueflame::game::gdt::FdString64 };
    (str256) => { blueflame::game::gdt::FdString256 };
    (vec2f) => { blueflame::game::gdt::FdVector2f };
    (vec3f) => { blueflame::game::gdt::FdVector3f };
    (vec4f) => { blueflame::game::gdt::FdVector4f };
    (bool[]) => { blueflame::game::gdt::FdBoolArray };
    (s32[]) => { blueflame::game::gdt::FdS32Array };
    (f32[]) => { blueflame::game::gdt::FdF32Array };
    (str64[]) => { blueflame::game::gdt::FdString64Array };
    (str256[]) => { blueflame::game::gdt::FdString256Array };
    (vec2f[]) => { blueflame::game::gdt::FdVector2fArray };
    (vec3f[]) => { blueflame::game::gdt::FdVector3fArray };
}

/// Helper trait for checking index for getter and setters
pub trait FlagIndex {
    fn to_index(self) -> Option<usize>;
}
#[rustfmt::skip]
const _: () = {
    impl FlagIndex for u8 { #[inline(always)] fn to_index(self) -> Option<usize> { Some(self as usize) } }
    impl FlagIndex for u16 { #[inline(always)] fn to_index(self) -> Option<usize> { Some(self as usize) } }
    impl FlagIndex for u32 { #[inline(always)] fn to_index(self) -> Option<usize> { Some(self as usize) } }
    impl FlagIndex for usize { #[inline(always)] fn to_index(self) -> Option<usize> { Some(self) } }
    impl FlagIndex for u64 { #[inline(always)] fn to_index(self) -> Option<usize> { 
        if self > usize::MAX as u64 {
            None
        } else {
            Some(self as usize)
        }
    }}
    impl FlagIndex for i8 { #[inline(always)] fn to_index(self) -> Option<usize> { (self as isize).to_index() } }
    impl FlagIndex for i16 { #[inline(always)] fn to_index(self) -> Option<usize> { (self as isize).to_index() } }
    impl FlagIndex for i32 { #[inline(always)] fn to_index(self) -> Option<usize> { (self as isize).to_index() } }
    impl FlagIndex for isize { #[inline(always)] fn to_index(self) -> Option<usize> { 
        if self < 0 {
            None
        } else {
            Some(self as usize)
        }
    } }
    impl FlagIndex for i64 { #[inline(always)] fn to_index(self) -> Option<usize> { 
        if self < 0 || self > usize::MAX as i64 {
            None
        } else {
            Some(self as usize)
        }
    } }
};

/// Trait for flag types that can be used in TriggerParam
pub trait FlagType: Clone + PartialEq {
    /// The shared storage type of the flag - each flag stores a shared value for the initial value
    type SharedType: Clone + PartialEq + 'static;
    /// The static storage type of the flag - used for initializing the flag value
    type StaticType: Clone + PartialEq + 'static;
    /// The value of the flag for array elements, used for minmax clamping and comparison
    type ValueType: Copy + PartialEq + 'static;

    fn from_shared(value: &Self::SharedType) -> Self;
    fn from_static(value: Self::StaticType) -> Self {
        Self::from_shared(&Self::static_to_shared(value))
    }
    fn static_to_shared(value: Self::StaticType) -> Self::SharedType;
    /// Clamps the value to the given min and max, used for bounds checking
    /// when setting values in TriggerParam.
    fn clamp(value: Self, min: Self::ValueType, max: Self::ValueType) -> Self;
    /// Returns a stub value for the type, used for initializing min/max
    /// that don't matter
    fn stub() -> Self::ValueType;
}

pub type StringFlagType = Arc<str>;
pub type ArrayFlagType<T> = Arc<[T]>;

#[rustfmt::skip]
const _: () = {
    macro_rules! scalar_impl {
        () => {
        type SharedType = Self; type StaticType = Self; type ValueType = Self;
        #[inline(always)] fn from_shared(value: &Self) -> Self { *value }
        #[inline(always)] fn static_to_shared(value: Self) -> Self { value }
        }
    }
    impl FlagType for bool {
        scalar_impl!();
        #[inline(always)] fn clamp(value: Self, _: Self, _: Self) -> Self { value }
        #[inline(always)] fn stub() -> Self { false }
    }
    impl FlagType for i32 {
        scalar_impl!();
        #[inline(always)] fn clamp(value: Self, min: Self, max: Self) -> Self { value.clamp(min, max) }
        #[inline(always)] fn stub() -> Self { 0 }
    }
    impl FlagType for f32 {
        scalar_impl!();
        #[inline(always)] fn clamp(value: Self, min: Self, max: Self) -> Self { value.clamp(min, max) }
        #[inline(always)] fn stub() -> Self { 0f32 }
    }
    impl FlagType for StringFlagType {
        type SharedType = Self; type StaticType = &'static str; type ValueType = &'static str;
        #[inline(always)] fn from_shared(value: &Self::SharedType) -> Self { Arc::clone(value) }
        #[inline(always)] fn from_static(value: Self::StaticType) -> Self { Arc::from(value) }
        #[inline(always)] fn static_to_shared(value: Self::StaticType) -> Self { Arc::from(value) }
        #[inline(always)] fn clamp(value: Self, _: Self::StaticType, _: Self::StaticType) -> Self { value }
        #[inline(always)] fn stub() -> Self::StaticType { "" }
    }
    impl FlagType for (f32, f32) {
        scalar_impl!();
        #[inline(always)] fn clamp(value: Self, min: Self::StaticType, max: Self::StaticType) -> Self { 
            (value.0.clamp(min.0, max.0), value.1.clamp(min.1, max.1))
        }
        #[inline(always)] fn stub() -> Self::StaticType { (0f32, 0f32) }
    }
    impl FlagType for (f32, f32, f32) {
        scalar_impl!();
        #[inline(always)] fn clamp(value: Self, min: Self::StaticType, max: Self::StaticType) -> Self { 
            (value.0.clamp(min.0, max.0), value.1.clamp(min.1, max.1), value.2.clamp(min.2, max.2))
        }
        #[inline(always)] fn stub() -> Self::StaticType { (0f32, 0f32, 0f32) }
    }
    impl FlagType for (f32, f32, f32, f32) {
        scalar_impl!();
        #[inline(always)] fn clamp(value: Self, min: Self::StaticType, max: Self::StaticType) -> Self { 
            (value.0.clamp(min.0, max.0), value.1.clamp(min.1, max.1), value.2.clamp(min.2, max.2), value.3.clamp(min.3, max.3))
        }
        #[inline(always)] fn stub() -> Self::StaticType { (0f32, 0f32, 0f32, 0f32) }
    }
    impl<T: FlagType + 'static> FlagType for ArrayFlagType<T> {
        type SharedType = ArrayFlagType<T>;
        type StaticType = &'static [T::StaticType]; 
        type ValueType = T::ValueType;
        #[inline(always)] fn from_shared(value: &Self::SharedType) -> Self {
            Self::clone(value)
        }
        #[inline(always)] fn static_to_shared(value: Self::StaticType) -> Self {
            let v = value.iter().map(|x| T::from_static(x.clone())).collect::<Vec<_>>();
            Self::from(v.as_slice())
        }
        #[inline(always)] fn clamp(value: Self, _: Self::ValueType, _: Self::ValueType) -> Self { value }
        #[inline(always)] fn stub() -> Self::ValueType { T::stub() }
    }
};

pub type FlagList<T> = Vec<Flag<T>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Flag<T: FlagType> {
    value: T,
    initial_value: T::SharedType,
    hash: i32,
    min: T::ValueType,
    max: T::ValueType,
    properties: u8,
}

enum PropertyFlag {
    ProgramReadable = 0x1,
    ProgramWritable = 0x2,
    Save = 0x4,
    // OneTrigger = 0x8,
    // EventAssociated = 0x10,
}

impl<T: FlagType> Flag<T> {
    pub fn new(
        // name: &'static str,
        hash: i32,
        initial_value: T::StaticType,
        properties: u8,
    ) -> Self {
        Self {
            value: T::from_static(initial_value.clone()),
            initial_value: T::static_to_shared(initial_value),
            hash,
            // name,
            min: T::stub(),
            max: T::stub(),
            properties,
        }
    }
    pub fn new_minmax(
        // name: &'static str,
        hash: i32,
        initial_value: T::StaticType,
        properties: u8,
        min: T::ValueType,
        max: T::ValueType,
    ) -> Self {
        Self {
            value: T::from_static(initial_value.clone()),
            initial_value: T::static_to_shared(initial_value),
            hash,
            // name,
            min,
            max,
            properties,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
    pub fn set(&mut self, value: T) {
        self.value = T::clamp(value, self.min, self.max);
    }
    pub fn reset(&mut self) {
        self.value = T::from_shared(&self.initial_value);
    }

    pub fn hash(&self) -> i32 {
        self.hash
    }

    pub fn max(&self) -> T::ValueType {
        self.max
    }

    pub fn readable(&self) -> bool {
        self.properties & (PropertyFlag::ProgramReadable as u8) != 0
    }

    pub fn writable(&self) -> bool {
        self.properties & (PropertyFlag::ProgramWritable as u8) != 0
    }

    pub fn savable(&self) -> bool {
        self.properties & (PropertyFlag::Save as u8) != 0
    }
}

impl<T: FlagType + 'static> Flag<ArrayFlagType<T>> {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn get_at<I: FlagIndex>(&self, idx: I) -> Option<&T> {
        self.value.get(idx.to_index()?)
    }
    #[must_use = "game implementation checks if the array index is valid"]
    pub fn reset_at<I: FlagIndex>(&mut self, idx: I) -> bool {
        let Some(i) = idx.to_index() else {
            return false;
        };
        let Some(init_value) = self.initial_value.get(i) else {
            return false;
        };
        let Some(x) = Arc::make_mut(&mut self.value).get_mut(i) else {
            return false;
        };
        *x = init_value.clone();
        true
    }
    #[must_use = "game implementation checks if the array index is valid"]
    pub fn set_at<I: FlagIndex>(&mut self, idx: I, value: T) -> bool {
        let Some(i) = idx.to_index() else {
            return false;
        };
        let Some(x) = Arc::make_mut(&mut self.value).get_mut(i) else {
            return false;
        };
        *x = value;
        true
    }
}

#[cfg(feature = "data")]
static BOOL_FLAG_PACK: &[u8] = include_bytes!("generated/bool_flag_pack.bin");
#[cfg(feature = "data")]
pub fn unpack_bool_flags() -> Vec<Flag<bool>> {
    if !BOOL_FLAG_PACK.len().is_multiple_of(5) {
        panic!("Invalid bool flag pack length");
    }
    let num_flags = BOOL_FLAG_PACK.len() / 5;
    let mut flags = Vec::with_capacity(num_flags);
    for i in 0..num_flags {
        let offset = i * 5;
        // first 4 bytes are the hash, in LE
        let hash = i32::from_le_bytes(BOOL_FLAG_PACK[offset..offset + 4].try_into().unwrap());
        // last byte is X00 FFFFF, X is initial value, FFFFF is the properties
        let last_byte = BOOL_FLAG_PACK[offset + 4];
        let initial_value = last_byte & 0x80 != 0;
        let properties = last_byte & 0x1F;
        flags.push(Flag::new(hash, initial_value, properties));
    }

    flags
}

#[cfg(feature = "data")]
static S32_FLAG_PACK: &[u8] = include_bytes!("generated/s32_flag_pack.bin");
#[cfg(feature = "data")]
pub fn unpack_s32_flags() -> Vec<Flag<i32>> {
    if !S32_FLAG_PACK.len().is_multiple_of(17) {
        panic!("Invalid s32 flag pack length");
    }

    let num_flags = S32_FLAG_PACK.len() / 17;
    let mut flags = Vec::with_capacity(num_flags);
    for i in 0..num_flags {
        let offset = i * 17;
        // first 16 bytes are 4 i32s in LE:
        // hash, initial_value, min, max
        // last byte is the property flags
        let hash = i32::from_le_bytes(S32_FLAG_PACK[offset..offset + 4].try_into().unwrap());
        let initial_value =
            i32::from_le_bytes(S32_FLAG_PACK[offset + 4..offset + 8].try_into().unwrap());
        let min = i32::from_le_bytes(S32_FLAG_PACK[offset + 8..offset + 12].try_into().unwrap());
        let max = i32::from_le_bytes(S32_FLAG_PACK[offset + 12..offset + 16].try_into().unwrap());
        // last byte is X00 FFFFF, X is initial value, FFFFF is the properties
        let properties = S32_FLAG_PACK[offset + 16] & 0x1F;
        flags.push(Flag::new_minmax(hash, initial_value, properties, min, max));
    }

    flags
}

#[cfg(test)]
#[cfg(feature = "data")]
mod tests {
    use super::*;

    #[test]
    fn test_unpack() {
        unpack_bool_flags();
        unpack_s32_flags();
    }
}
