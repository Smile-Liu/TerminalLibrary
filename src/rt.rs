#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut counter = Counter { value: 0 };
        normal_fn(&mut counter)
    }

    fn normal_fn(counter: &mut Counter) {
        counter.increment();
        assert_eq!(counter.value, 1);

        counter.decrement();
        assert_eq!(counter.value, 0);
    }
}


struct Counter {
    value: i32,
}
 
impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
 
    fn decrement(&mut self) {
        self.value -= 1;
    }
}