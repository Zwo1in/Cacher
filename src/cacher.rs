use std::collections::HashMap;

#[cfg(tests)]
mod test {
    use super::*;

    #[test]
    fn should_return_value_for_arg() {
        let mut cacher = Cacher::new(|arg| {
            arg
        });

        assert_eq!(cacher.value(2), 2);
        assert_eq!(cacher.value(3), 3);
        assert_eq!(cacher.value(4), 4);
    }
}

pub struct Cacher<T>
    where T: Fn(u32) -> u32
{
    fun: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(fun: T) -> Cacher<T> {
        Self {
            fun,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> &u32 {
        if self.value.contains_key(&arg) {
            return self.value.get(&arg).unwrap();
        }

        let v = (self.fun)(arg);
        self.value.insert(arg, v);
        self.value.get(&arg).unwrap()
    }
}