use std::vec::Vec;
use std::fmt::Debug;

fn emit<X: Debug>(x: X, s: &str) -> X {
    println!("{}: {:?}", s, x);
    x
}

fn emit_vec<X: Debug>(x: Vec<X>, s: &str) -> Vec<X> {
    println!("{}: {:?}", s, x);
    x
}

fn except<X: PartialEq + Clone>(x: Vec<X>, y: Vec<X>) -> Vec<X> {
    x.into_iter().filter(|item| !y.contains(item)).collect()
}

fn except_single<X: PartialEq + Clone>(x: Vec<X>, y: X) -> Vec<X> {
    x.into_iter().filter(|item| *item != y).collect()
}

fn exhaust<X: Clone + PartialEq + Debug + Into<usize>>(x: Vec<X>, y: X) -> Vec<X> {
    let mut i = y;
    let mut r = Vec::new();
    loop {
        let last = i.clone();
        i = x[i.into()].clone();
        if i == last {
            break;
        }
        r.push(i.clone());
    }
    r
}

fn _exhaust<X: Clone + PartialEq + Debug>(x: Vec<X>, y: usize) -> Vec<usize> {
    let mut i = y;
    let mut r = Vec::new();
    loop {
        let last = i;
        i = match x.get(i) {
            Some(value) => {
                // find the index of the value, if it exists in the vector
                x.iter().position(|x| x == value).unwrap_or(i)
            },
            None => break,
        };
        if i == last {
            break;
        }
        r.push(i);
    }
    r
}

fn til(n: usize) -> Vec<usize> {
    (0..n).collect()
}

#[derive(Debug)]
struct Tree<X> {
    x: Vec<X>,
    p: Vec<usize>,
}

impl<X> Tree<X>
where
    X: Clone,
{
    fn new() -> Self {
        Tree { x: Vec::new(), p: Vec::new() }
    }

    fn adopt(&mut self, parent: usize, child: usize) -> usize {
        self.p[child] = parent;
        child
    }

    fn insert(&mut self, parent: usize, item: X) -> usize {
        self.x.push(item);
        self.p.push(parent);
        self.x.len() - 1
    }

    fn parent(&self, child: usize) -> usize {
        self.p[child]
    }

    fn path(&self, child: usize) -> Vec<usize> {
        exhaust(self.p.clone(), child)
    }

    fn leaves(&self) -> Vec<usize> {
        except(til(self.p.len()), self.p.clone())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_except_with_first_element_removed() {
        let a = vec![1, 2, 3, 4, 5];
        let result = except(a.clone(), vec![a[0]]);
        assert_eq!(result, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_except_with_different_vector() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![2, 5, 30];
        let result = except(a, b);
        assert_eq!(result, vec![1, 3, 4]);
    }
    
    #[test]
    fn test_insert() {
        let mut t = Tree::new();
        let r = t.insert(0, "root");
        assert_eq!(r, 0);
        let child1 = t.insert(r, "child1");
        assert_eq!(child1, 1);
        let child2 = t.insert(r, "child2");
        assert_eq!(child2, 2);

        // Test the parent of the inserted nodes
        assert_eq!(t.parent(child1), r);
        assert_eq!(t.parent(child2), r);

        // Test the content of the nodes
        assert_eq!(t.x[child1], "child1");
        assert_eq!(t.x[child2], "child2");

        // Insert a child to one of the new nodes and test
        let grandchild = t.insert(child1, "grandchild");
        assert_eq!(grandchild, 3);
        assert_eq!(t.parent(grandchild), child1);
        assert_eq!(t.x[grandchild], "grandchild");
    }

    #[test]
    fn test_leaves() {
        let mut t = Tree::new();
        let r = t.insert(0, "root");
        let child1 = t.insert(r, "child1");
        let grandchild1 = t.insert(child1, "grandchild1"); // index 2
        let child2 = t.insert(r, "child2");
        let grandchild2 = t.insert(child2, "grandchild2"); // index 3

        let leaves = t.leaves();
        println!("Leaves: {:?}", leaves); // Debug output

        assert_eq!(leaves.len(), 2); // Check if there are two leave
        assert!(leaves.contains(&grandchild1), "Leaves does not contain grandchild1"); // Check specific leaves
        assert!(leaves.contains(&grandchild2), "Leaves does not contain grandchild1"); // Check specific leaves
    }

    #[test]
    fn test_path() {
        let mut t = Tree::new();
        let r = t.insert(0, "root");
        let child1 = t.insert(r, "child1");
        let grandchild1 = t.insert(child1, "grandchild1");

        let path_to_grandchild1 = t.path(grandchild1);
        assert_eq!(path_to_grandchild1, vec![child1, r]);

        // Test path to root (should be empty or contain only root based on implementation)
        let path_to_root = t.path(r);
        assert!(path_to_root.is_empty() || path_to_root == vec![r]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 999")]
    fn test_out_of_bounds() {
        let mut t = Tree::new();
        let r = t.insert(0, "root");
        let child1 = t.insert(r, "child1");
        let _grandchild1 = t.insert(child1, "grandchild1");
        // Test path to a node that doesn't exist (should handle this gracefully)
        let non_existing_node = 999;
        // this should panic
        let _path_to_non_existing = t.path(non_existing_node);
    }

    // todo: more tests!!
}
