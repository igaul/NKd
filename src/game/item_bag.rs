//ItemBag
// hashmap for holding inventory
pub type Bag = std::collections::HashMap<String, i32>;

#[derive(Debug, Clone)]
pub struct ItemBag {
    bag: Bag,
}

impl ItemBag {
    pub fn new() -> ItemBag {
        ItemBag {
            bag: Bag::with_capacity(0),
        }
    }
    //init with vec of strings
    pub fn gen_new(items: &[String]) -> ItemBag {
        let mut b = Bag::with_capacity(items.len());
        for s in items {
            b.insert(s.to_string(), 1);
        }
        ItemBag { bag: b }
    }
    //gen with values
    pub fn gen_bag_with_vals(items: &[(&str, i32)]) -> ItemBag {
        let mut b = Bag::with_capacity(items.len());
        for (k, v) in items {
            b.insert(k.to_string(), *v);
        }
        ItemBag { bag: b }
    }
    pub fn contains(&self, item: &str) -> bool {
        self.bag.contains_key(item)
    }
    //return count
    pub fn count(&self, item: &str) -> i32 {
        match self.bag.get(item) {
            Some(i) => *i,
            None => 0,
        }
    }
    //empty bag
    pub fn empty_bag(&mut self) {
        self.bag.clear()
    }
    //insert
    pub fn add(&mut self, item: &str) {
        if self.bag.contains_key(item) {
            self.bag.insert(item.to_string(), self.count(item) + 1);
        } else {
            self.bag.insert(item.to_string(), 1); //way around ???
        }
    }
    //remove
    pub fn remove(&mut self, item: &str) {
        if self.count(item) > 1 {
            self.bag.insert(item.to_string(), self.count(item) - 1);
        } else {
            self.bag.remove(item);
        }
    }
    //compare this to that, return first item this does not have or empty string //change to option ???
    pub fn compare(&self, other: &Self) -> String {
        for k in self.bag.keys() {
            if other.bag.contains_key(k) {
                return k.to_string();
            }
        }
        "".to_string()
    }
    //compare to tile reqs ( vec<string> )
    pub fn compare_to_tile_reqs(&self, other: &[String]) -> String {
        for k in other {
            if !self.contains(k) {
                return k.to_string();
            }
        }
        "".to_string()
    }
    //???
    pub fn contents_as_a_string(&self) -> String {
        let mut s = "".to_string();
        for (k, v) in self.bag.iter() {
            s.push_str(k);
            s.push_str(&" ".to_string());
            s.push_str(&v.to_string());
            s.push_str(&"\n".to_string());
        }
        s
    }
    pub fn contents_as_strings(&self) -> Vec<String> {
        let mut m: Vec<String> = Vec::with_capacity(self.bag.len());
        for (k, v) in self.bag.iter() {
            let mut t: String = k.to_string();
            t.push_str(&": ".to_string());
            t.push_str(&v.to_string());
            m.push(t);
        }
        m
    }
}

#[cfg(test)]
    mod tests {
        use super::*;
    #[test]
    fn test_item_bag() {
        let mut b = ItemBag::new();
        b.add("red towel");
        b.add("hamster");
        //copy contents as strings to test against sorted list
        let mut c_as_s = b.contents_as_strings();
        c_as_s.sort();
        assert_eq!(b.count("red towel"),1);
        assert_eq!(c_as_s,
                    ["hamster: 1","red towel: 1"]);

        b.add("red towel");
        b.add("hamster");
        let mut c_as_s = b.contents_as_strings();
        c_as_s.sort();
        assert_eq!(b.count("red towel"),2);
        assert_eq!(c_as_s,
                    ["hamster: 2","red towel: 2"]);

        b.remove("red towel");
        b.add("hamster");
        let mut c_as_s = b.contents_as_strings();
        c_as_s.sort();
        assert_eq!(b.count("red towel"),1);
        assert_eq!(c_as_s,
                    ["hamster: 3","red towel: 1"]);

        b.remove("red towel");
        b.add("hamster");
        assert_eq!(b.count("red towel"),0);
        assert_eq!(b.contents_as_strings(),
                    ["hamster: 4"]);

        b.empty_bag();
        assert_eq!(b.count("red towel"),0);
        assert!(b.contents_as_strings().is_empty());
    }
}


