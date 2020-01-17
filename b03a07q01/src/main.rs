// ASSIGNMENT 07 QUESTION 01
// Write a Rust Program,
// ● Make a module with suitable name in main.rs
// ● Make a sub module with an other name in first module
// ● Define a simple function in sub module
// ● Call that function from fn main

mod myinfo
{
    pub mod beautify
    {
        pub fn sl()
        {
            println!("---------------");
        }
        pub fn dl()
        {
            println!("===============");
        }
    }
    pub mod info
    {
        pub fn myinfo()
        {
            println!("Faisal Shahzad");
            println!("PIAIC49775");
        }
        pub fn batchinfo(batch:u8)
        {
            println!("PIAIC Batch {}",batch);
        }
        pub fn assginfo(assg:u8)
        {
            println!("Assignment No {}",assg);
        }
    }
}

fn main() {
    crate::myinfo::beautify::dl();
    crate::myinfo::info::myinfo();
    crate::myinfo::beautify::sl();
    crate::myinfo::info::batchinfo(3);
    crate::myinfo::beautify::sl();
    crate::myinfo::info::assginfo(7);
    crate::myinfo::beautify::dl();
}
