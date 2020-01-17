mod lib;

fn main() {
    lib::myinfo::mydl();
    println!("My Calculator!");
    lib::myinfo::mysl();
    println!("{} + {} = {}",50,20,lib::add(50,20));
    println!("{} - {} = {}",50,20,lib::sub(50,20));
    println!("{} * {} = {}",50,20,lib::mul(50,20));
    println!("{} / {} = {}",50,20,lib::div(50,20));
    println!("{} % {} = {}",50,20,lib::rem(50,20));
    lib::myinfo::mydl();
}
