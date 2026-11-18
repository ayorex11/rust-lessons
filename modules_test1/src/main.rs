pub mod shop{
    pub mod inventory {
        pub fn stock_count()-> u32{
            12
        }
        fn reorder_level()-> u32{ // private function but can be accessed by other child functions under inventory
            5
        }
        pub fn needs_reorder() -> bool{
            stock_count() < reorder_level() // example of it being called here despite not having the pub keyword
        }
    }
    pub mod pricing{
        pub fn discount()-> u32{
            10
        }
        pub fn stock_based_discount() -> u32{
            if super::inventory::stock_count() > 10{ // super calls shop essentially here 
                discount()
            } else{
                0
            }
        }
    }
}

fn main(){
    println!("stock: {}", crate::shop::inventory::stock_count()); // this will break if pub is removed from inventory
    println!("discount: {}", shop::pricing::stock_based_discount());
    println!("needs reorder: {}", shop::inventory::needs_reorder());// this will break if pub is removed from inventory
    //println!("{}", crate::shop::inventory::reorder_level()); // this won't work because reorder_level isn't a public function
}