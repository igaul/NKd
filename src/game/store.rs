//store.rs
//store sells items to player
//
//
use super::item_bag::ItemBag as Bag;
#[derive(Debug, Clone)]
pub struct Store {
    pub items: Bag,
    pub is_active: bool,
}
impl Store {
    pub fn gen_store() -> Store {
        Store {
            items: Bag::gen_bag_with_vals(&vec![
                ("(R)ope", 50),
                ("(H)ammer", 25),
                ("(B)oat", 75),
                ("(F)ace", 100),
            ]),
            is_active: false,
            //
        }
    }
    pub fn contents_to_strings(&self) -> Vec<String> {
        self.items.contents_as_strings()
    }
    pub fn purchase(&self, item: &str, wallet: &mut i32) -> bool {
        if self.items.contains(item) && *wallet >= self.items.count(item) {
            *wallet -= self.items.count(item);
            true
        } else {
            false
        }
    }
}
//purchase menu, game control
//tests
//collaspe into mod ???
