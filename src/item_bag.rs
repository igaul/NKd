//Item_Bag
//hashmap to hold items as String:count

pub type Bag = std::collections::HashMap<String, i32>; //or str??

pub struct Item_Bag {
    bag: Bag,
}
impl Item_Bag {
    pub fn new() -> Item_Bag {
        Item_Bag {
            bag: Bag::with_capacity(0)
        }
    }
    //init with vec of strings
    // pub fn new(items: &Vec<String>) -> Item_Bag {
    //     bag: Bag::with_capacity(items.len());
    //     for s in items {
    //         bag.add(s);
    //     }
    //     bag
    // }
    pub fn contains(&self, item: &String) -> bool { //or &String or &str ???
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
    pub fn empty(mut self) {
        self.bag.clear()
    }
    //insert
    pub fn add(mut self, item: &String) {
        if self.bag.contains_key(item) {
            self.bag.insert(item.to_string(), self.count(item) + 1);
        }
        else {
            self.bag.insert(item.to_string(), 1); //way around ???
        }
    }
    //compare this to that, return first item this does not have or empty string //change to result ???
    pub fn compare(&self, other: Bag) -> String {
        for k in self.bag.keys() {
            if other.contains_key(k) {
                return k.to_string()
            }
        }
        "".to_string()
    }
}