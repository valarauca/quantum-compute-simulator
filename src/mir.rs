use std::default::Default;
use super::parse::Data;
use super::seahash::SeaHasher;
use std::hash::{Hash,Hasher};


/// What position or generation is this comutation taking place in
#[derive(PartialEq,Eq,PartialOrd,Ord,Copy,Clone,Debug)]
pub struct Pos(pub usize);
impl Pos {
    /// Build a new position
    #[inline(always)]
    pub fn new(x: &usize) -> Self {
        Pos(x.clone())
    }
}

/// What variable is this related
#[derive(Copy,Clone,Debug)]
pub struct Tag(pub u64, u64);
impl Tag {
    /// Converts a string to a Tag
    pub fn new(s: &str) -> Self {
        let mut sea = SeaHasher::new();
        s.hash(&mut sea);
        Tag(sea.finish(),1)
    }
    /// Return's the size of a matrix
    pub fn size(&self) -> (u64,u64) {
        let val = 2 >> self.1;
        (val,val)
    }
    /// Double the size of an internal matrix
    pub fn increment_size(&mut self) {
        self.1 += 1;
    }
    /// Simulate an interaction between 2 qubits
    pub fn interact(&mut self, other: &mut Tag) {
        use std::cmp::Ordering;
        let mut larger = match self.1.cmp( &other.1) {
            Ordering::Less => other.1.clone(),
            _ => self.1.clone(),
        };
        larger += 1;
        self.1 = larger.clone();
        other.1 = larger.clone();
    }
}

/// What gate is being triggered
pub enum OpCode {
    Hadamard(Tag),
    Not(Tag),
    PauliY(Tag),
    PauliX(Tag),
    PauliZ(Tag),
    SqrtNot(Tag),
    PhaseGate(Tag,f64,f64),
    Swap(Tag,Tag),
    SqrtSwap(Tag,Tag),
    CntlNot(Tag,Tag),
    CntlZ(Tag,Tag),
    CntlX(Tag,Tag),
    CntlY(Tag,Tag),
    CntlPhase(Tag,Tag,f64,f64),
    Display(Tag)
}

/// Middle Intermediate Representation
pub enum MIR {
    Dec(Tag,Pos,f64,f64,f64,f64),
    Op(OpCode,Pos),
}
impl MIR {
    pub fn dec(a: &str, b: &usize, c:&f64, d: &f64, e: &f64, f: &f64) -> Self {
        MIR::Dec(Tag::new(a), Pos::new(b), c.clone(), d.clone(), e.clone(), f.clone())
    }
}
impl PartialEq for MIR {

}


macro_rules! dec_stuff {
    (@SIN $name: ident, $val: expr, $line: expr) => ( MIR::Op(OpCode::$name(Tag::new($val)), Pos::new($line)) );
    (@MUL $name: ident, $val0: expr, $val1: expr, $line: expr) => {
        MIR::Op(OpCode::$name(Tag::new($val0), Tag::new($val1)), Pos::new($line))
    };
}

/// Convert intput data into MIR
pub fn data_to_mir( x: &Data) -> MIR {
    match x {
        &Data::Let(ref val,ref a,ref b,ref c,ref d,ref line) => MIR::dec(val,line,a,b,c,d),
        &Data::Display(ref val, ref l) => dec_stuff!(@SIN Display, val, l),
        &Data::Hadamard(ref val, ref l) => dec_stuff!(@SIN Hadamard, val, l),
        &Data::Not(ref val, ref l) => dec_stuff!(@SIN Not, val, l),
        &Data::PauliY(ref val, ref l) => dec_stuff!(@SIN PauliY, val, l),
        &Data::PauliX(ref val, ref l) => dec_stuff!(@SIN PauliX, val, l),
        &Data::PauliZ(ref val, ref l) => dec_stuff!(@SIN PauliZ, val, l),
        &Data::SqrtNot(ref val, ref l) => dec_stuff!(@SIN SqrtNot, val, l),
        &Data::PhaseGate(ref val, ref a, ref b, ref line) => MIR::Op(OpCode::PhaseGate(Tag::new(val),a.clone(),b.clone()), Pos::new(line)),
        &Data::Swap(ref a, ref b, ref l) => dec_stuff!(@MUL Swap,a,b,l),
        &Data::SqrtSwap(ref a, ref b, ref l) => dec_stuff!(@MUL SqrtSwap,a,b,l),
        &Data::CntlNot(ref a, ref b, ref l) => dec_stuff!(@MUL CntlNot,a,b,l),
        &Data::CntlZ(ref a, ref b, ref l) => dec_stuff!(@MUL Swap,a,b,l),
        &Data::CntlX(ref a, ref b, ref l) => dec_stuff!(@MUL CntlX,a,b,l),
        &Data::CntlY(ref a, ref b, ref l) => dec_stuff!(@MUL CntlY,a,b,l),
        &Data::CntlPhase(ref a, ref b, ref c, ref d, ref line) => MIR::Op(OpCode::CntlPhase(Tag::new(a),Tag::new(b), c.clone(), d.clone()), Pos::new(line))
    }
}
