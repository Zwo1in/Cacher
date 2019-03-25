use std::collections::HashMap;
use std::hash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_value_for_arg() {
        let mut cacher = Cacher::new(|arg: &u32| {
            arg.clone()
        });

        assert_eq!(cacher.value(2), &2);
        assert_eq!(cacher.value(3), &3);
        assert_eq!(cacher.value(4), &4);
    }

    #[test]
    fn should_work_with_different_types() {
        let mut cacher = Cacher::new(|arg: &String| -> usize {
            arg.len()
        });

        assert_eq!(cacher.value("abc".to_string()), &3);
    }
}

pub struct Cacher<T, U, V>
    where T: Fn(&U) -> V,
          U: Eq + hash::Hash + Clone
{
    fun: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
    where T: Fn(&U) -> V,
          U: Eq + hash::Hash + Clone
{
    pub fn new(fun: T) -> Cacher<T, U, V> {
        Self {
            fun,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> &V {
        if self.value.contains_key(&arg) {
            return self.value.get(&arg).unwrap();
        }

        let v = (self.fun)(&arg);
        self.value.insert(arg.clone(), v);
        self.value.get(&arg).unwrap()
    }
}