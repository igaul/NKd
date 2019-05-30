//ItemBag
//hashmap to hold items as name<string>:count<i32>

pub type Bag = std::collections::HashMap<String, i32>; //or str??
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
    // pub fn gen_new(items: &Vec<String>) -> ItemBag {
    //     bag: Bag::with_capacity(items.len());
    //     for s in items {
    //         bag.add(s);
    //     }
    //     bag
    // }
    pub fn contains(&self, item: &String) -> bool {
        //or &String or &str ???
        self.bag.contains_key(item)
    }
    //return count
    pub fn count(&self, item: &String) -> i32 {
        match self.bag.get(item) {
            Some(i) => *i,
            None => 0,
        }
    }
    //empty bag
    pub fn empty_bag(mut self) {
        self.bag.clear()
    }
    //insert
    pub fn add(&mut self, item: &String) {
        if self.bag.contains_key(item) {
            self.bag.insert(item.to_string(), self.count(item) + 1);
        } else {
            self.bag.insert(item.to_string(), 1); //way around ???
        }
    }
    //remove
    pub fn remove(&mut self, item: &String) {
        if self.count(item) > 1 {
            self.bag.insert(item.to_string(), self.count(item) - 1);
        } else {
            self.bag.remove(item);
        }
    }
    //compare this to that, return first item this does not have or empty string //change to result ???
    pub fn compare(&self, other: Bag) -> String {
        for k in self.bag.keys() {
            if other.contains_key(k) {
                return k.to_string();
            }
        }
        "".to_string()
    }
    //compare to tile reqs ( vec<string> )
    pub fn compare_to_tile_reqs(&self, other: &Vec<String>) -> String { //change to result & match ???
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
        for (k,v) in self.bag.iter() {
            s.push_str(k);
            s.push_str(&" ".to_string());
            s.push_str(&v.to_string());
            s.push_str(&"\n".to_string());
        }
        s
    }
    pub fn contents_as_strings(&self) -> Vec<String> {
    let mut m: Vec<String> = Vec::with_capacity(self.bag.len());
    for (k,v) in self.bag.iter() {
        let mut t: String = k.to_string();
        t.push_str(&v.to_string());
        m.push(t);
    }
    m
    //self.bag.iter().map(|(x,y)| x.push_str(y.to_string())).collect()
    }
    // //returns vec<String,int> contents of bag
    // pub fn contents(&self) -> Vec<(String, i32)> {
    //     //self.bag.iter().cloned().map().collect()
    //     let mut m: Vec<(String, i32)> = Vec::with_capacity(self.bag.len());
    //     for (k,v) in self.bag.iter() {
    //         m.push((k.to_string(),*v))
    //     }
    //     m
    // }

   
    // #[test]
    // fn test_contents_as_strings() {
    //     let b = ItemBag::new();
    //     b.add("red towel");
    //     b.add("hamster");
    //     let s = b.contents_as_strings();
    //     assert_eq!(s,["red towel1","hamster1"]);
    // }

    // pub fn contents_to_string(&self) -> String {

    // }

}
