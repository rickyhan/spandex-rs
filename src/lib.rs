extern crate daggy;
use daggy::Dag;

trait Kind {
}

struct Id {id: usize}
impl Iterator for Id {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let ret = self.id;
        self.id += 1;
        Some(ret)
    }
}
impl Id{fn new()->Self{Self{id:0}}}

/// Variable Kind
struct Var<T> {
    value: T,
    set_at: usize,
}
impl<'a, T> Kind for Var<T> {
}
impl<T> Var<T> {
    fn new(value: T, set_at: usize) -> Self {
        Var {
            value,
            set_at,
        }
    }
}

struct Node {
    kind: Box<Kind>,
}

impl Node {
    fn new(kind: Box<Kind>) -> Self {
        Node {
            kind
        }
    }
}

/// Incremental
struct Incr {
    graph: Dag<Node, u32, usize>,
    node_id_counter: Id,
    stabilization_num_counter: Id,
    stabilization_num: usize,
}

impl Incr {
    fn new() -> Self {
        Incr {
            graph: Dag::new(),
            node_id_counter: Id::new(),
            stabilization_num_counter: Id::new(),
            stabilization_num: 0,
        }
    }
    fn var<T: 'static>(&mut self, value: T) -> Node {
        Node::new(
            Box::new(Var::new(value,
            self.stabilization_num))
        )
    }
}

fn test() {
    let mut incr = Incr::new();
    let a = incr.var(3);
    let b = incr.var("i");
}