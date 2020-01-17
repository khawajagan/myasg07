use libinput;
use libinfo;
use libcalc;

fn main() 
{
    libinfo::beauty::dl(15);
    println!("MY CALCULATOR");
    libinfo::beauty::dl(15);
    let mut mychoice:char='-';
    loop
    {
        let num1 = libinput::myinput::input::getisize("Enter 1st Number : ".to_string());
        let num2 = libinput::myinput::input::getisize("Enter 2nd Number : ".to_string());
        libinfo::beauty::sl(20);
        println!("a-ADD Numbers\nb-SUBTRACT Numbers\nm-MULTIPLY Numbers\nd-DIVIDE Numbers\nr-REMAINDER\nq-QUIT");
        let mychoice=libinput::myinput::input::getchar("Enter Your Choice".to_string());
        match mychoice
        {
            'a' => println!("{} + {} = {}",num1,num2,libcalc::calc::add(num1,num2)),
            's' => println!("{} - {} = {}",num1,num2,libcalc::calc::sub(num1,num2)),
            'm' => println!("{} x {} = {}",num1,num2,libcalc::calc::mul(num1,num2)),
            'd' => println!("{} / {} = {}",num1,num2,libcalc::calc::div(num1,num2)),
            'r' => println!("{} % {} = {}",num1,num2,libcalc::calc::rem(num1,num2)),
            'q' => break,
            _ => println!("Please Enter Valid Choice !"),
        }
    }
    let mytext = libinput::myinput::input::getstring("Test Entry".to_string());
    println!("Data Is {}", mytext);
    libinfo::beauty::sl(125);
    libinfo::info::myinfo();
}
