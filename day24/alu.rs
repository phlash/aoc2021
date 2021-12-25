use std::io::{self,BufRead};

#[derive(Debug)]
enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug,Copy,Clone)]
enum Reg {
    W = 0, X = 1, Y = 2, Z = 3,
}

#[derive(Debug,Copy,Clone)]
enum RegOrImm {
    Reg(Reg),
    Imm(isize),
    Ext,
}

#[derive(Debug)]
struct Inst {
    op : Op,
    tgt : Reg,
    src : RegOrImm,
}

fn emulate(prog : &Vec<Inst>, args : &[isize]) -> isize {
    let mut alu = [0,0,0,0];
    let mut arg : usize = 0;
    for pc in 0..prog.len() {
        // first get target reg
        let rt : usize = prog[pc].tgt as usize;
        // now source values
        let s1 : isize = alu[rt];
        let s2 : isize = match prog[pc].src {
            RegOrImm::Reg(r) => alu[r as usize],
            RegOrImm::Imm(v) => v,
            RegOrImm::Ext => { arg += 1; args[arg-1] },
        };
        // execute op
        alu[rt] = match prog[pc].op {
            Op::Inp => s2,
            Op::Add => s1 + s2,
            Op::Mul => s1 * s2,
            Op::Div => s1 / s2,
            Op::Mod => s1 % s2,
            Op::Eql => if s1==s2 { 1 } else { 0 },
        };
        //println!("alu: {:?}", alu);
    }
    return alu[3];
}

fn main() {
    let mut prog : Vec<Inst> = Vec::new();
    for line in io::stdin().lock().lines() {
        // each line is an instruction of the form: <op> <tgt> [<src|imm>]
        // where <src|imm> is implicit (from args) for 'inp'
        // <tgt> or <src> are registers, one of w,x,y,z
        let txt = line.unwrap();
        let toks : Vec<&str> = txt.split_whitespace().collect();
        if toks.len()<2 { println!("invalid: {}", txt); return; }
        let oc = match toks[0] {
            "inp"|"INP" => Op::Inp,
            "add"|"ADD" => Op::Add,
            "mul"|"MUL" => Op::Mul,
            "div"|"DIV" => Op::Div,
            "mod"|"MOD" => Op::Mod,
            "eql"|"EQL" => Op::Eql,
            _ => unreachable!(),
        };
        let r1 = match toks[1] {
            "w"|"W" => Reg::W,
            "x"|"X" => Reg::X,
            "y"|"Y" => Reg::Y,
            "z"|"Z" => Reg::Z,
            _ => unreachable!(),
        };
        let mut r2 = RegOrImm::Ext;     // assume external src unless specified
        if toks.len()>2 {
            r2 = match toks[2] {
                "w"|"W" => RegOrImm::Reg(Reg::W),
                "x"|"X" => RegOrImm::Reg(Reg::X),
                "y"|"Y" => RegOrImm::Reg(Reg::Y),
                "z"|"Z" => RegOrImm::Reg(Reg::Z),
                _ => RegOrImm::Imm( isize::from_str_radix(toks[2],10).unwrap() ),
            };
        }
        prog.push( Inst { op : oc, tgt : r1, src : r2 } );
    }
    println!("prog: {:?}", prog);
    // Part1 : search input space by adjusting each digit to minimize output..
    let mut model : [isize; 14] = [9,9,9,9,9,9,9,9,9,9,9,9,9,9];
    let mut done : bool = false;
    while !done {
        for digit in 0..model.len() {
            let mut min : isize = isize::MAX;
            let mut mdg : isize = model[digit];
            while model[digit]>0 {
                let z = emulate(&prog, &model);
                if z<min {
                    min = z;
                    mdg = model[digit];
                }
                model[digit] -= 1;
            }
            println!("digit: {} min: {}", digit, min);
            model[digit] = mdg;
            if 0==min { done = true; }
        }
    }
    println!("model {:?}", model);
}