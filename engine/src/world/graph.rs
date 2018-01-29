use std::collections::HashMap;

pub trait Unit {
    fn play(&self) -> usize;
}

pub struct Node {
    unit: Box<Unit>,
    next: Vec<u32>,
}

impl Node {
    pub fn play(&self) -> Option<u32> {
        self.next.get(self.unit.play()).map(|n: &u32| *n)
    }
}

pub struct Graph {
    map: HashMap<u32, Node>,
    next: Option<u32>,
}

impl Graph {
    pub fn next(&mut self) {
        self.next = self.next
            .and_then(|n: u32| self.map.get(&n))
            .and_then(|node| node.play())
            .and_then(|n: u32| if self.map.contains_key(&n) { Some(n) } else { None });
    }

    pub fn init(&mut self, next: Option<u32>) {
        self.next = next;
    }

    pub fn more(&self) -> bool {
        self.next.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestUnit {
        next: usize,
    }
    impl Unit for TestUnit {
        fn play(&self) -> usize {
            println!("choosing {} next", self.next);
            self.next
        }
    }

    #[test]
    fn test_node() {
        let t = Box::new(TestUnit{next: 1});
        assert!(t.play() == 1);
        let n = Node{
            unit: t,
            next: vec![3, 4],
        };
        assert!(n.play() == Some(4));
    }

    #[test]
    fn test_graph() {
        let t0 = Box::new(TestUnit{next: 0});
        let t1 = Box::new(TestUnit{next: 0});
        let t2 = Box::new(TestUnit{next: 0});
        let t3 = Box::new(TestUnit{next: 0});

        let mut map = HashMap::new();
        map.insert(0, Node{unit: t0, next: vec![1]});
        map.insert(1, Node{unit: t1, next: vec![2]});
        map.insert(2, Node{unit: t2, next: vec![3]});
        map.insert(3, Node{unit: t3, next: vec![4]});

        let mut graph = Graph{map: map, next: None};

        // Set next to the node at 0
        graph.init(Some(0));
        assert!(graph.more() == true);

        // Move to node 0 to 1
        graph.next();
        assert!(graph.more() == true);

        // Move to node 1 to 2
        graph.next();
        assert!(graph.more() == true);

        // Move to node 2 to 3
        graph.next();
        assert!(graph.more() == true);

        // Try 3 to 4, should fail
        graph.next();
        assert!(graph.more() == false);
    }
}
