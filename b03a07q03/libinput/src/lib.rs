use std::io;
pub mod myinput 
{
    pub mod display 
    {
        pub fn myprint(text:String)
        {
            println!("{}",text);
        }
    }
    pub mod input 
    {
        pub fn getstring(mytext: String) -> String 
        {
            crate::myinput::display::myprint(mytext);
            let mut text = String::new();
            std::io::stdin().read_line(&mut text).expect("Please Enter Data");
            return text;
        }
        pub fn getu8(mytext: String) -> u8
        {
            let mynum:u8 = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
        pub fn getu16(mytext: String) -> u16
        {
            let mynum:u16 = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
        pub fn getu32(mytext: String) -> u32
        {
            let mynum:u32 = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
        pub fn getu64(mytext: String) -> u64
        {
            let mynum:u64 = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
	pub fn getusize(mytext: String) -> usize
        {
            let mynum:usize = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
        pub fn geti8(mytext: String) -> i8
        {
            let mynum:i8 = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
        pub fn geti16(mytext: String) -> i16
        {
            let mynum:i16 = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
        pub fn geti32(mytext: String) -> i32
        {
            let mynum:i32 = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
        pub fn geti64(mytext: String) -> i64
        {
            let mynum:i64 = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
	pub fn getisize(mytext: String) -> isize
        {
            let mynum:isize = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Numeric Data Please");
            return mynum;
        }
        pub fn getchar(mytext: String) -> char
        {
            let mynum:char = (crate::myinput::input::getstring(mytext)).trim().parse().expect("Enter Single Character Please");
            return mynum;
        }

    }
}
