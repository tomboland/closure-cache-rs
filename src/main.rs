use std::fmt;
use std::collections::BTreeMap;
use std::cell::RefCell;

struct CloCache<U, V>
where
    U: Fn(V) -> V,
    V: fmt::Display + Ord + Copy,
{
    closure: U,
    result_map: RefCell<BTreeMap<V, V>>,
}

impl<U, V: fmt::Display> CloCache<U, V>
where
    U: Fn(V) -> V,
    V: fmt::Display + Ord + Copy,
{
    fn new(closure: U) -> CloCache<U, V> {
        CloCache {
            closure,
            result_map: RefCell::new(BTreeMap::<V, V>::new()),
        }
    }

    fn value(&mut self, arg: V) -> V {
        let mut result_map = self.result_map.borrow_mut();
        match result_map.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.closure)(arg);
                result_map.insert(arg, v);
                v.clone()
            }
        }
    }
}


fn main() {
    let mut uint = CloCache::new(|x: u32| x + 1);
    println!("uint: {}", uint.value(0));
    let mut iint = CloCache::new(|x: i32| x - 1);
    println!("iint: {}", iint.value(0));
    ()
}
