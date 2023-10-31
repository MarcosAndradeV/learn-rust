use std::ops::Add;


#[derive(Debug)]
struct Counter {
    count: u64
}

impl Add for Counter {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Counter {count: (self.count + rhs.count) as u64}
    }
}

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

#[cfg(test)]
mod test {
    use super::Counter;

    #[test]
    pub fn test_counter_1(){
        let c1 = Counter { count: 8 };
        let c2 = Counter { count: 4 };
        assert_eq!(c1 + c2, Counter { count: 12 });
    }
    
    #[test]
    pub fn test_counter_2(){
        let c1 = Counter { count: 8 };
        let c2 = Counter { count: 4 };
        assert!(c1 != c2);
        let c3 = Counter { count: 8 };
        assert!(c1 == c3);

    }

}