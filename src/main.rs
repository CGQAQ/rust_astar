use std::cell::RefCell;
use std::rc::{Rc, Weak};

const INFINITY: i32 = i32::MAX;

fn main() {
    let mut nodes = init(10, 10);
    init_neighbors(10, 10, &mut nodes);
    let path = astar(
        Rc::downgrade(nodes.get(0).unwrap()),
        Rc::downgrade(nodes.get(10 * 10 - 1).unwrap()),
        |a, b| {
            let a = a.upgrade().unwrap().clone();
            let b = b.upgrade().unwrap().clone();
            let c = (((a.borrow().p.x - b.borrow().p.x).pow(2)
                + (a.borrow().p.y - b.borrow().p.y).pow(2)) as f64)
                .sqrt()
                .floor() as i32;
            return c;
        },
    );
    println!("{:?}", path)
}

fn astar<F>(
    start: Weak<RefCell<Node>>,
    end: Weak<RefCell<Node>>,
    h: F,
) -> Option<Vec<Weak<RefCell<Node>>>>
where
    F: Fn(Weak<RefCell<Node>>, Weak<RefCell<Node>>) -> i32,
{
    let mut openSet: Vec<Weak<RefCell<Node>>> = vec![start.clone()];

    // init start point
    let s = start.upgrade().unwrap();
    let gs = h(start.clone(), end.clone());
    let mut sr = s.borrow_mut();
    sr.fscore = 0;
    sr.gscore = gs;
    drop(sr);
    drop(s);

    // start algorithm
    while !openSet.is_empty() {
        openSet.sort_by_key(|k| k.upgrade().unwrap().borrow().fscore);
        let current = openSet.first().clone().unwrap();
        let end = &end;
        if current
            .upgrade()
            .unwrap()
            .borrow()
            .eq(&*(end.clone()).upgrade().unwrap().borrow())
        {
            // if current == end, finish finding
            return Some(reconstruct_path(current.clone()));
        }

        current
            .upgrade()
            .unwrap()
            .borrow()
            .neighbors
            .iter()
            .for_each(|v| {
                // tentative_gScore := gScore[current] + d(current, neighbor)
                let mut openSet = openSet.clone();
                let tentative_gScore = current.upgrade().unwrap().borrow().gscore + 1;
                if tentative_gScore < v.upgrade().unwrap().borrow().gscore {
                    // This path to neighbor is better than any previous one. Record it!
                    let neighbor = v.upgrade().unwrap();
                    let gs = h(v.clone(), end.clone());
                    let mut g = neighbor.borrow_mut();
                    g.parent = Some(current.clone());
                    g.gscore = tentative_gScore;
                    g.fscore = g.gscore + gs;
                    drop(g);
                    drop(neighbor);
                    if openSet
                        .iter()
                        .any(|i| i.clone().upgrade().as_ref() == v.clone().upgrade().as_ref())
                    {
                        openSet.push(v.clone())
                    }
                }
            });
        openSet.remove(0);
    }
    None
}

fn reconstruct_path(current: Weak<RefCell<Node>>) -> Vec<Weak<RefCell<Node>>> {
    // function reconstruct_path(cameFrom, current)
    //     total_path := {current}
    //     while current in cameFrom.Keys:
    //         current := cameFrom[current]
    //         total_path.prepend(current)
    //     return total_path
    let mut result_path: Vec<Weak<RefCell<Node>>> = vec![];
    result_path.push(current.clone());
    let mut current = current;

    while current.clone().upgrade().unwrap().borrow().parent.is_some() {
        current = current.upgrade().unwrap().borrow().parent.clone().unwrap();
        result_path.push(current.clone());
    }
    return result_path;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    p: Point,
    parent: Option<Weak<RefCell<Node>>>,
    neighbors: Vec<Weak<RefCell<Node>>>,
    fscore: i32,
    gscore: i32,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            p: Point { x: 0, y: 0 },
            parent: None,
            neighbors: vec![],
            fscore: INFINITY,
            gscore: INFINITY,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.p.x == other.p.x && self.p.y == other.p.y
    }
}

fn init_neighbors(width: i32, height: i32, nodes: &mut Vec<Rc<RefCell<Node>>>) {
    assert!((width * height) as usize == nodes.len());
    for y in 0..height {
        for x in 0..width {
            let np = vec![
                (x - 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x + 1, y + 1),
            ];
            let np: Vec<(i32, i32)> = np
                .into_iter()
                .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
                .collect();
            let nps: Vec<Weak<RefCell<Node>>> = np
                .into_iter()
                .map(|(x, y)| Rc::downgrade(nodes.get((x * y + x) as usize).unwrap()))
                .collect();
            nps.iter().for_each(|r| {
                nodes
                    .get((x * y + x) as usize)
                    .cloned()
                    .unwrap()
                    .borrow_mut()
                    .neighbors
                    .push(r.clone())
            });
        }
    }
}

fn init(width: i32, height: i32) -> Vec<Rc<RefCell<Node>>> {
    assert!(width > 0);
    assert!(height > 0);
    let mut result: Vec<Rc<RefCell<Node>>> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            result.push(Rc::new(RefCell::new(Node {
                p: Point { x, y },
                parent: None,
                neighbors: vec![],
                fscore: INFINITY,
                gscore: INFINITY,
            })))
        }
    }
    return result;
}
