pub mod beauty
{
    pub fn sl(n:u8)
    {
        for i in 1..=n
        {print!("-")};
        println!("");
    }
    pub fn dl(n:u8)
    {
        for i in 1..=n
        {print!("=")};
        println!("");
    }
}
pub mod info
{
    pub fn myinfo()
    {
        crate::beauty::dl(20);
        println!("BATCH 03 ISLAMABAD");
        crate::beauty::sl(20);
        println!("ROLL NO : PIAIC-49775");
        println!("NAME : FAISAL SHAHZAD");
        crate::beauty::dl(20);
    }
}