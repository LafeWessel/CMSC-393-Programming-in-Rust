
struct big_int<'a>{
    head : digit<'a>,
    tail : digit<'a>,
}

struct digit<'a> {
    num : u8,
    next : &'a mut digit<'a>,
    prev : &'a mut digit<'a>,
}