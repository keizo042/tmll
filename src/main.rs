fn main() {
    let mut v: Vec<L> = vec![];
    let mut tm = State::new(&mut v, 0);
    println!("{}", tm.start());
}

#[derive(Clone)]
struct State<'a> {
    tape: &'a [L],
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
    Eight,
    Nine,
    Ten,
}

#[derive(Clone)]
enum L {
    Blunk,
    X,
    One,
    Zero,
    W,
}

impl<'a> State<'a> {
    fn new(l: &'a mut [L], i: usize) -> State {
        return State { tape: l, ptr: i };
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

    fn step(&mut self, s: S) -> S {
        let p = self.ptr;
        let n = self.tape[p].clone();
        match s {
            S::Init => match n {
                L::Blunk => S::Rej,
                L::W => {
                    self.ptr += 1;
                    return S::Two;
                },
                _ => S::One,
            },
            S::One => {
                match n {
                    L::Zero => S::Three,
                    L::One => S::Six,
                    L::W => S::Rej, /* not yet implmenet */
                    _ => {
                        return S::Rej;
                    },
                }
            },
            S::Two => {
                match n {
                    L::Blunk => return S::Acc,
                    _ => S::Rej,
                }
            },
            S::Three => match n {
                L::Zero | L::One => {
                    self.ptr += 1;
                    return S::Three;
                },
                L::Blunk => {
                    self.ptr -= 1;
                    return S::Four;
                },
                _ => S::Rej,
            },
            S::Four => match n {
                L::Blunk => {
                    self.ptr -= 1;
                    return S::Four;
                },
                L::Zero => {
                    let p = self.ptr;
                    let t = self.tape.clone();
                    t[p] = L::Blunk;
                    self.ptr -= 1;
                    return S::Five;
                },
                _ => S::Rej,
            },
            S::Five => match n {
                L::Blunk => {
                    self.ptr += 1;
                    return S::One;
                },
                _ => {
                    self.ptr -= 1;
                    return S::Five;
                },
            },
            S::Six => match n {
                L::Zero | L::One => {
                    self.ptr += 1;
                    return S::Six;
                },
                L::Blunk => {
                    self.ptr -= 1;
                    return S::Seven;
                },
                _ => S::Rej,
            },
            S::Seven => match n {
                L::One => {
                    self.tape[self.ptr] = L::Blunk;
                    self.ptr -= 1;
                    return S::Five;
                },
                _ => {
                    return S::Rej;
                },
            },
            S::Eight => S::Rej,
            S::Nine => S::Rej,
            S::Ten => S::Rej,
            _ => S::Rej,
        }
    }
}




#[test]
fn test1() {
    let mut v = [L::Blunk, L::Zero, L::One, L::W, L::One, L::Zero, L::Blunk];
    let mut n: usize = 0;
    let mut tm = State::new(&mut v, 0);

    assert_eq!(true, tm.start());
}

#[test]
fn test2() {
    let mut v = [L::Blunk, L::One, L::W, L::Zero, L::Blunk];
    let mut tm = State::new(&mut v, 0);
    assert_eq!(false, tm.start());
}

#[test]
fn test3() {
    let mut v = [L::Blunk];
    let mut tm = State::new(&mut v, 0);
    assert_eq!(false, tm.start());
}

#[test]
fn test4() {
    let mut v = [L::Blunk, L::W, L::Blunk];
    let mut tm = State::new(&mut v, 0);
    assert_eq!(true, tm.start());
}
