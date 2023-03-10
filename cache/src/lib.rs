pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 1);
}

#[test]
fn is_iter_cleared() {
    let v1 = vec!["abc", "abcd", "abc"];
    let mut v2: Vec<&str> = Vec::new();

    let v_iter = v1.iter();
    println!("{:#?}", v_iter);
    for v in v_iter {
        v2.push(*v);
    }

    // println!("{:#?}", v_iter); sho
    assert!(false);
}
