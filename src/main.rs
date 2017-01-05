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
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[derive(Clone)]
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
    //    --> One -----------------------| --> Two ------> Reject
    //    |    |                         |           |
    //    |   0,1                        |           |
    //    |    |                         |           |
    //    |  Three<---|                 Six<--|      |---> Accept
    //    |    |--*---|                  |----|
    //    |    |                         |
    //    |    |                         |
    //    |    |                         |
    //    |  blunk                    blunk 
    //    |    |                         |
    //    |    |                         |
    //    |    |                         |
    //    |   Four --*--> Reject       Seven -*-> Reject
    //    |    |                         |
    //    |    |                         |
    //    |    |                         |
    //    |-- Five<----------------------|
    //        |  |
    //        ----
    //
    fn step(&mut self, s: S) -> S {
        let p = self.ptr;
        let n = self.tape[p].clone();
        match s {
            S::Init => {
                match n {
                    L::Blunk => S::Rej,
                    _ => S::One,
                }
            }
            S::One => {
                match n {
                    L::Zero => S::Three,
                    L::One => S::Six,
                    L::X => {
                        self.right();
                        return S::One;
                    }
                    L::Blunk => {
                        return S::Rej;
                    }
                }
            }
            S::Two => {
                match n {
                    L::Blunk => return S::Acc,
                    _ => S::Rej,
                }
            }
            S::Three => {
                match n {
                    L::Blunk => {
                        self.left();
                        return S::Four;
                    }
                    _ => {
                        self.right();
                        return S::Three;
                    }
                }
            }
            S::Four => {
                match n {
                    L::X => {
                        self.left();
                        return S::Four;
                    }
                    L::Zero => {
                        let p = self.ptr;
                        let mut t = self.tape.clone();
                        t[p] = L::X;
                        self.left();
                        return S::Five;
                    }
                    _ => S::Rej,
                }
            }
            S::Five => {
                match n {
                    L::Blunk => {
                        self.right();
                        return S::One;
                    }
                    _ => {
                        self.left();
                        return S::Five;
                    }
                }
            }
            S::Six => {
                match n {
                    L::Blunk => {
                        self.left();
                        return S::Seven;
                    },
                    _ => {
                        self.right();
                        return S::Six;
                    }
                }
            }
            S::Seven => {
                match n {
                    L::One => {
                        let p = self.ptr;
                        let mut t = self.tape.clone();
                        t[p] = L::X;
                        self.left();
                        return S::Five;
                    }
                    L::X => {
                        self.left();
                        return S::Seven;
                    },
                    _ => {
                        return S::Rej;
                    }
                }
            },
            _ => S::Rej,
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
    let mut v = vec![ L::Blunk, L::Zero,  L::Zero, L::Blunk];
    let mut tm = State::new(&mut v, 1);
    assert_eq!(true, tm.start());
}

