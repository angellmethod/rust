use std::iter;

pub type Domino = (usize, usize);

/// A table keeping track of available dominoes.
///
/// Effectively a 6x6 matrix. Each position denotes whether a domino is available with that column
/// dots and row dots. Positions are mirrored ((3,4) == (4,3)), except for positions with equal row
/// and column numbers.
struct AvailabilityTable {
    m: Vec<usize>
}

impl AvailabilityTable {
    fn new() -> AvailabilityTable {
        AvailabilityTable { m: iter::repeat(0).take(6 * 6).collect() }
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.m[(x-1) * 6 + (y-1)]
    }

    fn set(&mut self, x: usize, y: usize, v: usize) {
        let m = &mut self.m[..];
        m[(x-1) * 6 + (y-1)] = v;
    }

    fn add(&mut self, x: usize, y: usize) {
        if x == y {
            let n = self.get(x, y);
            self.set(x, y, n + 1) // Along the diagonal
        }
        else {
            let m = self.get(x, y);
            self.set(x, y, m + 1);
            let n = self.get(y, x);
            self.set(y, x, n + 1);
        }
    }

    fn remove(&mut self, x: usize, y: usize) {
        if self.get(x, y) > 0 {
            if x == y {
                let n = self.get(x, y);
                self.set(x, y, n - 1) // Along the diagonal
            }
            else {
                let m = self.get(x, y);
                self.set(x, y, m - 1);
                let n = self.get(y, x);
                self.set(y, x, n - 1);
            }
        }
        else {
            // For this toy code hard explicit fail is best
            panic!("remove for 0 stones: ({:?}, {:?})", x, y)
        }
    }
    
    fn pop_first(&mut self, x: usize) -> Option<usize> {
        for y in 1 .. 7 {
            if self.get(x, y) > 0 {
                self.remove(x, y);
                return Some(y)
            }
        }
        None
    }
}

pub fn chain(dominoes: &Vec<Domino>) -> Option<Vec<Domino>> {
    match dominoes.len() {
        0 => Some(vec!()),
        1 => if dominoes[0].0 == dominoes[0].1 { Some(vec![dominoes[0]]) } else { None },
        _ => {
            // First check if the total number of each amount of dots is even, if not it's not
            // possible to complete a cycle. This follows from that it's an Eulerian path.
            let mut v: Vec<usize> = vec!(0, 0, 0, 0, 0, 0);
            // Keep the mutable borrow in a small scope here to allow v.iter().
            {
                let vs = &mut v[..];
                for dom in dominoes.iter() {
                    vs[dom.0 - 1] += 1;
                    vs[dom.1 - 1] += 1;
                }
            }
            for n in v.iter() {
                if n % 2 != 0 {
                    return None
                }
            }
            let chain = chain_worker(dominoes);
            if chain.len() == dominoes.len() {
                Some(chain)
            }
            else {
                None
            }
        }
    }
}

fn chain_worker(dominoes: &Vec<Domino>) -> Vec<Domino> {
    let mut doms = dominoes.clone();
    let first = doms.pop().unwrap();
    let mut t = AvailabilityTable::new();
    for dom in doms.iter() {
        t.add(dom.0, dom.1)

    }
    let mut v: Vec<Domino> = Vec::new();
    v.push(first);
    let mut n = first.1; // Number to connect to
    while let Some(m) = t.pop_first(n) {
        v.push((n, m));
        n = m;
    }
    v
}