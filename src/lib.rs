// #![warn(cast_possible_truncation)]
// #![warn(cast_possible_wrap)]
// #![warn(cast_sign_loss)]
// #![warn(filter_map)]
// #![warn(if_not_else)]
// #![warn(items_after_statements)]
// #![warn(nonminimal_bool)]
// #![warn(option_map_unwrap_or)]
// #![warn(option_map_unwrap_or_else)]
// #![warn(option_unwrap_used)]
// #![warn(shadow_reuse)]
// #![warn(shadow_same)]
// #![warn(shadow_unrelated)]
// #![warn(single_match_else)]
// #![warn(wrong_pub_self_convention)]

pub struct CappedMultiset {
    elements: Vec<u8>,
    cap: u8,
}

impl CappedMultiset {
    pub fn new(item: Vec<u8>) -> CappedMultiset {
        CappedMultiset {
            elements: item,
            cap: u8::max_value()
        }
    }

    pub fn sum(&self) -> u8 {
        let mut sum = 0;
        for elem in self.elements.iter().map(|&x| std::cmp::min(x, self.cap)) {
            println!("{}", elem);
            sum += elem;
        }
        sum
    }

    pub fn set_cap(&mut self, cap: Option<u8>) {
        self.cap = cap.unwrap_or(u8::max_value());
    }
}

#[cfg(test)]
mod tests {
    use CappedMultiset;
    #[test]
    fn test_sum() {
        let simple_array: Vec<u8> = vec![1,2,3,4,5];
        let mut testset = CappedMultiset::new(simple_array);
        assert_eq!(testset.sum(), 15);
        testset.set_cap(Some(3));
        assert_eq!(testset.sum(), 12);
        testset.set_cap(None);
        assert_eq!(testset.sum(), 15);
        testset.set_cap(Some(1));
        assert_eq!(testset.sum(), 5);
        testset.set_cap(Some(0));
        assert_eq!(testset.sum(), 0);
    }
}
