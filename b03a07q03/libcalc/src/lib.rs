pub mod calc
{
    pub fn add(n1:isize,n2:isize)->isize
    {
        n1+n2
    }
    pub fn sub(n1:isize,n2:isize)->isize
    {
        n1-n2
    }
    pub fn mul(n1:isize,n2:isize)->isize
    {
        n1*n2
    }
    pub fn div(n1:isize,n2:isize)->isize
    {
        if n2 != 0 { n1/n2 } else { 0 }
    }
    pub fn rem(n1:isize,n2:isize)->isize
    {
        if n2 != 0 { n1%n2 } else { 0 }
    }
}