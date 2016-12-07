

use super::nom;
use super::nom::IResult;
use std::str;
use std::str::FromStr;

//What I'm loading is a valid UTF-8 string
//I then convert it to a byte array for nom
//So if I can't recover the utf8 string,
//there is a big problem
#[inline(always)]
fn to_str<'a>(x: &'a [u8]) -> &'a str {
    match str::from_utf8(x) {
        Ok(x) => x,
        Err(_) => unreachable!()
    }
}

///Tokens for the Tokenizer! Identifiers for the Identifier throne!
///
///These items are used to build the AST. There is also _some_ 
#[derive(Debug,Clone,PartialEq,Eq)]
enum Keyword<'a> {
    Comment,
    Let(&'a str, &'a [u8], &'a [u8], &'a [u8], &'a [u8]),
    Display(&'a str),
    Hadamard(&'a str),
    Not(&'a str),
    PauliY(&'a str),
    PauliX(&'a str),
    PauliZ(&'a str),
    SqrtNot(&'a str),
    PhaseGate(&'a str, &'a [u8], &'a [u8]),
    Swap(&'a str, &'a str),
    SqrtSwap(&'a str, &'a str),
    CntlNot(&'a str, &'a str),
    CntlZ( &'a str, &'a str),
    CntlX( &'a str, &'a str),
    CntlY( &'a str, &'a str),
    CntlPhase(&'a str, &'a str, &'a [u8], &'a [u8]),
}
///Actual Tokens to work with
#[derive(Debug,Clone)]
pub enum Data<'a> {
    Let(&'a str,f64,f64,f64,f64,usize),
    Display(&'a str,usize),
    Hadamard(&'a str,usize),
    Not(&'a str,usize),
    PauliY(&'a str,usize),
    PauliX(&'a str,usize),
    PauliZ(&'a str,usize),
    SqrtNot(&'a str,usize),
    PhaseGate(&'a str,f64,f64,usize),
    Swap(&'a str, &'a str,usize),
    SqrtSwap(&'a str, &'a str,usize),
    CntlNot(&'a str, &'a str,usize),
    CntlZ( &'a str, &'a str,usize),
    CntlX( &'a str, &'a str,usize),
    CntlY( &'a str, &'a str,usize),
    CntlPhase(&'a str, &'a str,f64,f64,usize),
}
#[inline(always)]
fn error_and_exit(line_num: usize, line: &str, msg: &str) -> ! {
    println!("\n\nCompiler Error");
    println!("Syntax Error occured.");
    println!("{}", msg);
    println!("Error on line {:?}", line_num);
    println!("{:?}", line);
    println!("\nSome generic tips:");
    println!("Floating point values MUST have a decimal");
    println!("\tSo \"1\" is an error, while \"1.\" isn't");
    println!("\tNo space between \"+i\" on let statements");
    println!("\tNo space between the crappy ket notation always \"|0>\" or \"|1>\"");
    ::std::process::exit(0);
}

#[inline(always)]
fn afc(x: &[u8],line_num: usize, line: &str) -> f64 {
    match FromStr::from_str(to_str(x)) {
        Ok(z) => z,
        Err(e) => {
            println!("\n\nCompiler Error");
            println!("Failed to convert a value to a float.");
            println!("On line: {:?}", line_num);
            println!("{}",line);
            println!("Rust Error: {:?}", e);
            ::std::process::exit(0);
        }
    }
}
///Parse some code!!!!
pub fn parser<'a>(s: &'a str) -> Vec<Data<'a>> {
    s
    .lines()
    .enumerate()
    .map(|x| (x.0+1,x.1.trim()))
    .filter(|x| x.1.len() != 0)
    .map(|x| (x.0,x.1,parse_quantum(x.1.trim().as_bytes())) )
    .map(|x| {
        match x.2 {
            IResult::Done(_,val) => (x.0,x.1,val),
            _ => error_and_exit(x.0,x.1,"Syntax Error! (TODO: Improve this message)"),
        }})
    .filter(|x| x.2 != Keyword::Comment)
    .map(|x| {
        let n = x.0;
        let l = x.1;
        match x.2 {
            Keyword::Let(a,b,c,d,e) => Data::Let(a,afc(b,n,l),afc(c,n,l),afc(d,n,l),afc(e,n,l),n),
            Keyword::Display(a) => Data::Display(a,n),
            Keyword::Hadamard(a) => Data::Hadamard(a,n),
            Keyword::Not(a) => Data::Not(a,n),
            Keyword::PauliY(a) => Data::PauliY(a,n),
            Keyword::PauliX(a) => Data::PauliX(a,n),
            Keyword::PauliZ(a) => Data::PauliZ(a,n),
            Keyword::SqrtNot(a) => Data::SqrtNot(a,n),
            Keyword::PhaseGate(a,b,c) => Data::PhaseGate(a,afc(b,n,l),afc(c,n,l),n),
            Keyword::Swap(a,b) => Data::Swap(a,b,n),
            Keyword::SqrtSwap(a,b) => Data::SqrtSwap(a,b,n),
            Keyword::CntlNot(a,b) => Data::CntlNot(a,b,n),
            Keyword::CntlZ(a,b) => Data::CntlZ(a,b,n),
            Keyword::CntlY(a,b) => Data::CntlY(a,b,n),
            Keyword::CntlX(a,b) => Data::CntlX(a,b,n),
            Keyword::CntlPhase(a,b,c,d) => Data::CntlPhase(a,b,afc(c,n,l),afc(d,n,l),n),
            Keyword::Comment => panic!("This can't happen")
        }})
    .collect()
}




named!(parse_quantum<Keyword>,alt!(
    parse_comment |
    parse_let |
    phase |
    parse_single |
    parse_two |
    cntl_phase
));

named!( float<&[u8]>, complete!(recognize!(chain!(
        tag!("-")? ~
        take_while!(nom::is_digit)~
        tag!(".")? ~
        take_while!(nom::is_digit)?, ||{}
))));
named!(make_str<&str>, map!(take_while!(nom::is_alphabetic), to_str));
named!(parse_comment<Keyword>,chain!( tag!("//"), || Keyword::Comment ));
named!(parse_let<Keyword>, chain!(
    tag!("let") ~
    take_while!(nom::is_space)~
    var_: make_str~
    take_while!(nom::is_space)~
    tag!("=")~
    take_while!(nom::is_space)?~
    a: float~
    take_while!(nom::is_space)?~
    tag!("+i")~
    take_while!(nom::is_space)?~
    b: float~
    take_while!(nom::is_space)?~
    tag!("|0>")~
    take_while!(nom::is_space)?~
    c: float~
    take_while!(nom::is_space)?~
    tag!("+i")~
    take_while!(nom::is_space)?~
    d: float~
    take_while!(nom::is_space)?~
    tag!("|1>")~
    take_while!(nom::is_space)?~
    tag!(";"),
    || Keyword::Let(var_,a,b,c,d)
));
named!(cntl_phase<Keyword>, chain!(
    tag!("control_phase")~
    take_while!(nom::is_space)?~
    tag!("(")~ 
    take_while!(nom::is_space)?~
    var1: make_str~
    take_while!(nom::is_space)?~
    tag!(",")~
    var2: make_str~
    take_while!(nom::is_space)?~
    tag!(",")~
    take_while!(nom::is_space)?~
    num: float~
    take_while!(nom::is_space)?~
    tag!(",")~
    take_while!(nom::is_space)?~
    den: float~
    take_while!(nom::is_space)?~
    tag!(")")~ 
    take_while!(nom::is_space)?~
    tag!(";"),
    || Keyword::CntlPhase(var1,var2,num,den)
));
named!(phase<Keyword>, chain!(
    tag!("phase")~
    take_while!(nom::is_space)?~
    tag!("(")~ 
    take_while!(nom::is_space)?~
    var1: make_str~
    take_while!(nom::is_space)?~
    tag!(",")~
    take_while!(nom::is_space)?~
    num: float~
    take_while!(nom::is_space)?~
    tag!(",")~
    take_while!(nom::is_space)?~
    den: float~
    take_while!(nom::is_space)?~
    tag!(")")~ 
    take_while!(nom::is_space)?~
    tag!(";"),
    || Keyword::PhaseGate(var1,num,den)
));
named!(parse_single<Keyword>, chain!(
        keyword: alt!(
            tag!("display") |
            tag!("had") |
            tag!("not") |
            tag!("paulix") |
            tag!("pauliy") |
            tag!("pauliz") |
            tag!("sqrtnot") 
        ) ~
        take_while!(nom::is_space)?~
        tag!("(")~
        take_while!(nom::is_space)?~
        var_: make_str~
        take_while!(nom::is_space)?~
        tag!(")")~
        take_while!(nom::is_space)?~
        tag!(";"),
        || {
            match keyword {
                b"display" => Keyword::Display(var_),
                b"had" => Keyword::Hadamard(var_),
                b"not" => Keyword::Not(var_),
                b"paulix" => Keyword::PauliX(var_),
                b"pauliy" => Keyword::PauliY(var_),
                b"pauliz" => Keyword::PauliZ(var_),
                b"sqrtnot" => Keyword::SqrtNot(var_),
                _ => unreachable!()
            }
        }
));
named!(parse_two<Keyword>, chain!(
        keyword: alt!(
            tag!("swap") |
            tag!("sqrt_swap") |
            tag!("control_not") |
            tag!("control_x") |
            tag!("control_y") |
            tag!("control_z") 
        ) ~
        take_while!(nom::is_space)?~
        tag!("(")~
        take_while!(nom::is_space)?~
        arg1: make_str~
        take_while!(nom::is_space)?~
        tag!(",")~
        take_while!(nom::is_space)?~
        arg2: make_str~
        take_while!(nom::is_space)?~
        tag!(")")~
        take_while!(nom::is_space)?~
        tag!(";"),
        || {
            match keyword {
                b"swap" => Keyword::Swap(arg1,arg2),
                b"sqrt_swap" => Keyword::SqrtSwap(arg1,arg2),
                b"control_not" => Keyword::CntlNot(arg1,arg2),
                b"control_x" => Keyword::CntlX(arg1,arg2),
                b"control_y" => Keyword::CntlY(arg1,arg2),
                b"control_z" => Keyword::CntlZ(arg1,arg2),
                _ => unreachable!()
            }
        }
));


/***********************************************************************************
 *
 * Below this line are tests
 *
 **********************************************************************************/

macro_rules! gen_test {
    ($dut_str: expr, $dut_val: expr, $func: ident) => (
        let dut = $dut_str;
        let (_,val) = $func(dut).unwrap();
        assert_eq!($dut_val, val);
    )
}
#[test]
fn test_comment_parse() {
    gen_test!(b"//",Keyword::Comment,parse_comment);
    gen_test!(b"//The Parser should ignore everything after that",Keyword::Comment,parse_comment);
}
#[test]
fn parse_floating_point() {
    gen_test!(b"-3.14159",b"-3.14159",float);
    gen_test!(b"-0.14159",b"-0.14159",float);
    gen_test!(b"3.14159",b"3.14159",float);
    gen_test!(b"3.",b"3.",float);
    gen_test!(b"0.1",b"0.1",float);
}
#[test]
fn test_parse_two() {
    gen_test!(b"control_not ( hello , world ) ;",Keyword::CntlNot("hello","world"),parse_two);
    gen_test!(b"control_x(a,b);",Keyword::CntlX("a","b"),parse_two);
    gen_test!(b"control_y( a,b );",Keyword::CntlY("a","b"),parse_two);
    gen_test!(b"swap( a , b ) ;",Keyword::Swap("a","b"),parse_two);
    gen_test!(b"sqrt_swap( Aza,ZGd);",Keyword::SqrtSwap("Aza","ZGd"),parse_two);
    gen_test!(b"control_z(a,b);",Keyword::CntlZ("a","b"),parse_two);
}
#[test]
fn test_display() {
    gen_test!(b"had(hello);",Keyword::Hadamard("hello"),parse_single);
    gen_test!(b"not(a);",Keyword::Not("a"),parse_single);
    gen_test!(b"display(a);",Keyword::Display("a"),parse_single);
    gen_test!(b"pauliz(a);",Keyword::PauliZ("a"),parse_single);
    gen_test!(b"pauliy(a);",Keyword::PauliY("a"),parse_single);
    gen_test!(b"paulix(a);",Keyword::PauliX("a"),parse_single);
    gen_test!(b"sqrtnot(a);",Keyword::SqrtNot("a"),parse_single);
}
#[test]
fn test_let() {
    gen_test!(b"let myqubit=0.0+i -1.0 |0> 0.3 +i 1.0|1> ;",
        Keyword::Let("myqubit",b"0.0",b"-1.0",b"0.3",b"1.0"),
        parse_let);
    gen_test!(b"let myqubit=0.0+i-1.0|0>0.3+i1.0|1>;",
        Keyword::Let("myqubit",b"0.0",b"-1.0",b"0.3",b"1.0"),
        parse_let);
    gen_test!(b"let myqubit  =  0.0  +i  -1.0  |0>  0.3  +i  1.0  |1>  ;",
        Keyword::Let("myqubit",b"0.0",b"-1.0",b"0.3",b"1.0"),
        parse_let);
}
#[test]
fn test_phasegate() {
    gen_test!(b"phase(myqubit,1.,-2.);",Keyword::PhaseGate("myqubit",b"1.",b"-2."),phase);
}
#[test]
fn test_cntl_phase() {
    gen_test!(b"control_phase(qubitA,qubitB,1.,4.);", Keyword::CntlPhase("qubitA","qubitB",b"1.",b"4."),cntl_phase);
}
