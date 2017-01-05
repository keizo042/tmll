fn main() {
    let mut v: Vec<L> = vec![L::Blunk, L::One, L::One, L::Blunk];
    let mut tm = State::new(&mut v, 1);
    println!("{}", tm.start());
}

#[derive(Clone)]
struct State<'a> {
    tape: &'a Vec<L>,
    ptr: usize,
}

enum S {
    Acc,
    Rej,
    Init,
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Clone,Copy)]
enum L {
    Blunk,
    X,
    One,
    Zero,
}

impl<'a> State<'a> {
    fn new(l: &'a mut Vec<L>, i: usize) -> State {
        return State { tape: l, ptr: i };
    }

    fn left(&mut self) {
        self.ptr-=1;
    }

    fn right(&mut self) {
        self.ptr += 1;
    }

    fn start(&mut self) -> bool {
        let mut state = S::Init;
        loop {
            match state {
                S::Acc => return true,
                S::Rej => return false,
                _ => state = self.step(state),
            }
        }
    }

    //
    // State Transimition Diagram
    //
    //       Init
    //         |<-|
    //         |  X
    //
    //

    fn step(&mut self, s: S) -> S {
        let p = self.ptr;
        let n = self.tape[p].clone();
        match s {
            S::Init => {
                match n {
                    L::Blunk => S::Rej,
                    _ => S::A,
                }
            },
            S::A => {
                match n {
                    L::X => {
                        self.right();
                        return S::A;
                    },
                    L::Blunk => {
                        return S::Acc;
                    },
                    L::Zero => {
                        let mut tape = self.tape.clone();
                        tape[p] = L::X;
                        return S::B;
                    },
                    L::One => {
                        let mut tape = self.tape.clone();
                        let p = self.ptr;
                        tape[p] = L::X;
                        return S::D;
                    },
                }
            },
            // match Zero
            S::B => {
                match n {
                L::Blunk => {
                    self.left();
                    return S::C;
                },
                _ => {
                    self.right();
                    return S::B;
                }
                }

            },
            S::C => {
                match n {
                L::X => {
                    self.left();
                    return S::C;
                }
                L::Zero => {
                    let mut tape = self.tape.clone();
                    let p = self.ptr;
                    tape[p] = L::X;
                    return S::F;
                },
                _ => {
                    return S::Rej;
                },
                }

            },
            // match One
            S::D => {
                match n {
                L::Blunk => {
                    self.left();
                    return S::E;
                },
                _ => {
                    self.right();
                    return S::D;
                },
                }
            },
            S::E => {
                match n {
                L::X => {
                    self.left();
                    return S::E;
                }
                L::One => {
                    let mut tape = self.tape.clone();
                    let p = self.ptr;
                    tape[p] = L::X;
                    return S::F;
                }
                _ => {
                    return S::Rej;
                }
                }
            },
            S::F => {
                match n {
                L::Blunk => {
                    self.right();
                    return S::A;
                }
                _ => {
                    self.left();
                    return S::F;
                },
                }
            }
            _ => {
                return S::Rej;
            }
        }
    }
}




#[test]
fn test1() {
    let mut v = vec![L::Blunk, L::Zero, L::One, L::One, L::Zero, L::Blunk];
    let mut tm = State::new(&mut v, 1);

    assert_eq!(true, tm.start());
}

#[test]
fn test2() {
    let mut v = vec![L::Blunk, L::One, L::Zero, L::Blunk];
    let mut tm = State::new(&mut v, 1);
    assert_eq!(false, tm.start());
}

#[test]
fn test3() {
    let mut v = vec![L::Blunk, L::Blunk];
    let mut tm = State::new(&mut v, 1);
    assert_eq!(false, tm.start());
}

#[test]
fn test4() {
    let mut v = vec![L::Blunk, L::Blunk];
    let mut tm = State::new(&mut v, 1);
    assert_eq!(true, tm.start());
}

#[test]
fn test5() {
    let mut v = vec![ L::Blunk, L::One,   L::One, L::Blunk];
    let mut tm = State::new(&mut v, 1);
    assert_eq!(true, tm.start());

}

#[test]
fn test6() {
    let mut v = vec![ L::Blunk,  L::Zero, L::Blunk];
    let mut tm = State::new(&mut v, 1);
    assert_eq!(false, tm.start());
}


