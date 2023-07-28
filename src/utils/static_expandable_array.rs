use std::ops::Index;

const BASE_POWER: usize = 4;
const BASE: usize = 1 << BASE_POWER;

pub struct StaticExpandableArray<T> where T: Default + Copy {
    length: usize,
    _s: Storage<T>,
}

impl<T> StaticExpandableArray<T> where T: Default + Copy {
    pub fn new() -> Self {
        Self {
            length: 0,
            _s: Storage::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        let segment = get_segment(self.length);

        match segment {
            0 => self._s._0.get_or_insert_with(|| Box::new([T::default(); BASE << 0]))[self.length] = item,
            1 => self._s._1.get_or_insert_with(|| Box::new([T::default(); BASE << 1]))[self.length - get_lower_index_range(1)] = item,
            2 => self._s._2.get_or_insert_with(|| Box::new([T::default(); BASE << 2]))[self.length - get_lower_index_range(2)] = item,
            3 => self._s._3.get_or_insert_with(|| Box::new([T::default(); BASE << 3]))[self.length - get_lower_index_range(3)] = item,
            4 => self._s._4.get_or_insert_with(|| Box::new([T::default(); BASE << 4]))[self.length - get_lower_index_range(4)] = item,
            5 => self._s._5.get_or_insert_with(|| Box::new([T::default(); BASE << 5]))[self.length - get_lower_index_range(5)] = item,
            6 => self._s._6.get_or_insert_with(|| Box::new([T::default(); BASE << 6]))[self.length - get_lower_index_range(6)] = item,
            7 => self._s._7.get_or_insert_with(|| Box::new([T::default(); BASE << 7]))[self.length - get_lower_index_range(7)] = item,
            8 => self._s._8.get_or_insert_with(|| Box::new([T::default(); BASE << 8]))[self.length - get_lower_index_range(8)] = item,
            9 => self._s._9.get_or_insert_with(|| Box::new([T::default(); BASE << 9]))[self.length - get_lower_index_range(9)] = item,
            10 => self._s._10.get_or_insert_with(|| Box::new([T::default(); BASE << 10]))[self.length - get_lower_index_range(10)] = item,
            11 => self._s._11.get_or_insert_with(|| Box::new([T::default(); BASE << 11]))[self.length - get_lower_index_range(11)] = item,
            12 => self._s._12.get_or_insert_with(|| Box::new([T::default(); BASE << 12]))[self.length - get_lower_index_range(12)] = item,
            13 => self._s._13.get_or_insert_with(|| Box::new([T::default(); BASE << 13]))[self.length - get_lower_index_range(13)] = item,
            14 => self._s._14.get_or_insert_with(|| Box::new([T::default(); BASE << 14]))[self.length - get_lower_index_range(14)] = item,
            15 => self._s._15.get_or_insert_with(|| Box::new([T::default(); BASE << 15]))[self.length - get_lower_index_range(15)] = item,
            16 => self._s._16.get_or_insert_with(|| Box::new([T::default(); BASE << 16]))[self.length - get_lower_index_range(16)] = item,
            17 => self._s._17.get_or_insert_with(|| Box::new([T::default(); BASE << 17]))[self.length - get_lower_index_range(17)] = item,
            18 => self._s._18.get_or_insert_with(|| Box::new([T::default(); BASE << 18]))[self.length - get_lower_index_range(18)] = item,
            19 => self._s._19.get_or_insert_with(|| Box::new([T::default(); BASE << 19]))[self.length - get_lower_index_range(19)] = item,
            20 => self._s._20.get_or_insert_with(|| Box::new([T::default(); BASE << 20]))[self.length - get_lower_index_range(20)] = item,
            21 => self._s._21.get_or_insert_with(|| Box::new([T::default(); BASE << 21]))[self.length - get_lower_index_range(21)] = item,
            22 => self._s._22.get_or_insert_with(|| Box::new([T::default(); BASE << 22]))[self.length - get_lower_index_range(22)] = item,
            23 => self._s._23.get_or_insert_with(|| Box::new([T::default(); BASE << 23]))[self.length - get_lower_index_range(23)] = item,
            24 => self._s._24.get_or_insert_with(|| Box::new([T::default(); BASE << 24]))[self.length - get_lower_index_range(24)] = item,
            25 => self._s._25.get_or_insert_with(|| Box::new([T::default(); BASE << 25]))[self.length - get_lower_index_range(25)] = item,
            26 => self._s._26.get_or_insert_with(|| Box::new([T::default(); BASE << 26]))[self.length - get_lower_index_range(26)] = item,
            27 => self._s._27.get_or_insert_with(|| Box::new([T::default(); BASE << 27]))[self.length - get_lower_index_range(27)] = item,
            28 => self._s._28.get_or_insert_with(|| Box::new([T::default(); BASE << 28]))[self.length - get_lower_index_range(28)] = item,
            29 => self._s._29.get_or_insert_with(|| Box::new([T::default(); BASE << 29]))[self.length - get_lower_index_range(29)] = item,
            _ => panic!("Unexpected segment!"),
        }

        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            None
        } else {
            let segment = get_segment(index);
    
            match segment {
                0 => self._s._0.as_ref().and_then(|array| array.get(index)),
                1 => self._s._1.as_ref().and_then(|array| array.get(index - get_lower_index_range(1))),
                2 => self._s._2.as_ref().and_then(|array| array.get(index - get_lower_index_range(2))),
                3 => self._s._3.as_ref().and_then(|array| array.get(index - get_lower_index_range(3))),
                4 => self._s._4.as_ref().and_then(|array| array.get(index - get_lower_index_range(4))),
                5 => self._s._5.as_ref().and_then(|array| array.get(index - get_lower_index_range(5))),
                6 => self._s._6.as_ref().and_then(|array| array.get(index - get_lower_index_range(6))),
                7 => self._s._7.as_ref().and_then(|array| array.get(index - get_lower_index_range(7))),
                8 => self._s._8.as_ref().and_then(|array| array.get(index - get_lower_index_range(8))),
                9 => self._s._9.as_ref().and_then(|array| array.get(index - get_lower_index_range(9))),
                10 => self._s._10.as_ref().and_then(|array| array.get(index - get_lower_index_range(10))),
                11 => self._s._11.as_ref().and_then(|array| array.get(index - get_lower_index_range(11))),
                12 => self._s._12.as_ref().and_then(|array| array.get(index - get_lower_index_range(12))),
                13 => self._s._13.as_ref().and_then(|array| array.get(index - get_lower_index_range(13))),
                14 => self._s._14.as_ref().and_then(|array| array.get(index - get_lower_index_range(14))),
                15 => self._s._15.as_ref().and_then(|array| array.get(index - get_lower_index_range(15))),
                16 => self._s._16.as_ref().and_then(|array| array.get(index - get_lower_index_range(16))),
                17 => self._s._17.as_ref().and_then(|array| array.get(index - get_lower_index_range(17))),
                18 => self._s._18.as_ref().and_then(|array| array.get(index - get_lower_index_range(18))),
                19 => self._s._19.as_ref().and_then(|array| array.get(index - get_lower_index_range(19))),
                20 => self._s._20.as_ref().and_then(|array| array.get(index - get_lower_index_range(20))),
                21 => self._s._21.as_ref().and_then(|array| array.get(index - get_lower_index_range(21))),
                22 => self._s._22.as_ref().and_then(|array| array.get(index - get_lower_index_range(22))),
                23 => self._s._23.as_ref().and_then(|array| array.get(index - get_lower_index_range(23))),
                24 => self._s._24.as_ref().and_then(|array| array.get(index - get_lower_index_range(24))),
                25 => self._s._25.as_ref().and_then(|array| array.get(index - get_lower_index_range(25))),
                26 => self._s._26.as_ref().and_then(|array| array.get(index - get_lower_index_range(26))),
                27 => self._s._27.as_ref().and_then(|array| array.get(index - get_lower_index_range(27))),
                28 => self._s._28.as_ref().and_then(|array| array.get(index - get_lower_index_range(28))),
                29 => self._s._29.as_ref().and_then(|array| array.get(index - get_lower_index_range(29))),
                _ => None,
            }
        }
    }
}

impl<T: Default + Copy> Index<usize> for StaticExpandableArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(val) => val,
            None => panic!("Out of bounds access"),
        }
    }
}

struct Storage<T> {
    // _X: Option<Box<[T; BASE << X]>>, Lower_Index_Range_X: BASE * (2 ^ X - 1), Upper_Index_Range_X: Lower_Index_Range_X + BASE << X - 1
    _0: Option<Box<[T; BASE << 0]>>, // indexes 0-15
    _1: Option<Box<[T; BASE << 1]>>, // indexes 16-47    
    _2: Option<Box<[T; BASE << 2]>>, // indexes 48-111
    _3: Option<Box<[T; BASE << 3]>>, // indexes 112-239
    _4: Option<Box<[T; BASE << 4]>>,
    _5: Option<Box<[T; BASE << 5]>>,
    _6: Option<Box<[T; BASE << 6]>>,
    _7: Option<Box<[T; BASE << 7]>>,
    _8: Option<Box<[T; BASE << 8]>>,
    _9: Option<Box<[T; BASE << 9]>>,
    _10: Option<Box<[T; BASE << 10]>>,
    _11: Option<Box<[T; BASE << 11]>>,
    _12: Option<Box<[T; BASE << 12]>>,
    _13: Option<Box<[T; BASE << 13]>>,
    _14: Option<Box<[T; BASE << 14]>>,
    _15: Option<Box<[T; BASE << 15]>>,
    _16: Option<Box<[T; BASE << 16]>>,
    _17: Option<Box<[T; BASE << 17]>>,
    _18: Option<Box<[T; BASE << 18]>>,
    _19: Option<Box<[T; BASE << 19]>>,
    _20: Option<Box<[T; BASE << 20]>>,
    _21: Option<Box<[T; BASE << 21]>>,
    _22: Option<Box<[T; BASE << 22]>>,
    _23: Option<Box<[T; BASE << 23]>>,
    _24: Option<Box<[T; BASE << 24]>>,
    _25: Option<Box<[T; BASE << 25]>>,
    _26: Option<Box<[T; BASE << 26]>>,
    _27: Option<Box<[T; BASE << 27]>>,
    _28: Option<Box<[T; BASE << 28]>>,
    _29: Option<Box<[T; BASE << 29]>>,
    // _30: Option<Box<[T; BASE << 30]>>,
    // Past this point for BASE = 16 you might get overflows idk
    // _31: Option<Box<[T; BASE << 31]>>,
    // _32: Option<Box<[T; BASE << 32]>>,
    // _33: Option<Box<[T; BASE << 33]>>,
    // _34: Option<Box<[T; BASE << 34]>>,
    // _35: Option<Box<[T; BASE << 35]>>,
    // _36: Option<Box<[T; BASE << 36]>>,
    // _37: Option<Box<[T; BASE << 37]>>,
    // _38: Option<Box<[T; BASE << 38]>>,
    // _39: Option<Box<[T; BASE << 39]>>,
    // _40: Option<Box<[T; BASE << 40]>>,
    // _41: Option<Box<[T; BASE << 41]>>,
    // _42: Option<Box<[T; BASE << 42]>>,
    // _43: Option<Box<[T; BASE << 43]>>,
    // _44: Option<Box<[T; BASE << 44]>>,
    // _45: Option<Box<[T; BASE << 45]>>,
    // _46: Option<Box<[T; BASE << 46]>>,
    // _47: Option<Box<[T; BASE << 47]>>,
    // _48: Option<Box<[T; BASE << 48]>>,
    // _49: Option<Box<[T; BASE << 49]>>,
    // _50: Option<Box<[T; BASE << 50]>>,
    // _51: Option<Box<[T; BASE << 51]>>,
    // _52: Option<Box<[T; BASE << 52]>>,
    // _53: Option<Box<[T; BASE << 53]>>,
    // _54: Option<Box<[T; BASE << 54]>>,
    // _55: Option<Box<[T; BASE << 55]>>,
    // _56: Option<Box<[T; BASE << 56]>>,
    // _57: Option<Box<[T; BASE << 57]>>,
    // _58: Option<Box<[T; BASE << 58]>>,
    // _59: Option<Box<[T; BASE << 59]>>,
    // _60: Option<Box<[T; BASE << 60]>>,
    // _61: Option<Box<[T; BASE << 61]>>,
    // _62: Option<Box<[T; BASE << 62]>>,
    // _63: Option<Box<[T; BASE << 63]>>,
    // Technically should be expanded for a 128 bit system, but it shouldn't be my problem
}

impl<T> Storage<T> {
    fn new() -> Self {
        Self {
            _0: None,
            _1: None,
            _2: None,
            _3: None,
            _4: None,
            _5: None,
            _6: None,
            _7: None,
            _8: None,
            _9: None,
            _10: None,
            _11: None,
            _12: None,
            _13: None,
            _14: None,
            _15: None,
            _16: None,
            _17: None,
            _18: None,
            _19: None,
            _20: None,
            _21: None,
            _22: None,
            _23: None,
            _24: None,
            _25: None,
            _26: None,
            _27: None,
            _28: None,
            _29: None,
            // _30: None,
            // _31: None,
            // _32: None,
            // _33: None,
            // _34: None,
            // _35: None,
            // _36: None,
            // _37: None,
            // _38: None,
            // _39: None,
            // _40: None,
            // _41: None,
            // _42: None,
            // _43: None,
            // _44: None,
            // _45: None,
            // _46: None,
            // _47: None,
            // _48: None,
            // _49: None,
            // _50: None,
            // _51: None,
            // _52: None,
            // _53: None,
            // _54: None,
            // _55: None,
            // _56: None,
            // _57: None,
            // _58: None,
            // _59: None,
            // _60: None,
            // _61: None,
            // _62: None,
            // _63: None,
        }
    }
}

fn get_segment(index: usize) -> usize {
    let value = index + BASE;
    let leading_zeros = value.leading_zeros() as usize;
    let bits = std::mem::size_of::<usize>() * 8;
    bits - leading_zeros - BASE_POWER - 1
}

fn get_lower_index_range(segment: usize) -> usize {
    BASE * ((1usize  << segment).wrapping_sub(1))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_segment() {
        assert_eq!(get_segment(10), 0);
        assert_eq!(get_segment(15), 0);
        assert_eq!(get_segment(16), 1);
        assert_eq!(get_segment(47), 1);
        assert_eq!(get_segment(48), 2);
        assert_eq!(get_segment(111), 2);
        assert_eq!(get_segment(112), 3);
    }

    #[test]
    fn test_get_lower_index_range() {
        assert_eq!(get_lower_index_range(0), 0);
        assert_eq!(get_lower_index_range(1), 16);
        assert_eq!(get_lower_index_range(2), 48);
        assert_eq!(get_lower_index_range(3), 112);
    }

    #[test]
    fn test_push_and_get() {
        let mut array: StaticExpandableArray<i32> = StaticExpandableArray::new();

        // Test initial conditions
        assert_eq!(array.get(0), None);

        // Push some values and test the retrieval
        for i in 0..200 {
            array.push(i);
            assert_eq!(array.get(i as usize).cloned(), Some(i));
        }

        // Check out-of-bounds behavior
        assert_eq!(array.get(200), None);
    }

    #[test]
    #[should_panic(expected = "Out of bounds access")]
    fn test_out_of_bounds_index() {
        let array: StaticExpandableArray<i32> = StaticExpandableArray::new();
        let _ = array[0];
    }
}