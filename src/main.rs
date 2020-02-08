use std::collections::VecDeque;

fn main() {
    let mut t = Tree::new(10);
    t.print();
    t.add(1);
    t.add(15);
    t.add(7);
    t.add(100);
    t.add(0);
    t.add(100);
    t.print();

    test(&t, 1);
    test(&t, 200);
    test(&t, 100);
}

fn test(t : &Tree, x :i64) {
    if t.find(x) {
        println!("{} exists", x);
    } else {
        println!("{} is not found", x);
    }
}

#[derive(Default)]
struct Tree {
    root: i64,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    fn new(root: i64) -> Tree {
        Tree {
            root: root,
            ..Default::default()
        }
    }

    fn print(&self) {
        println!("{}", self.root);

        let mut q = VecDeque::<&Option<Box<Tree>>>::new();
        let mut q_next = VecDeque::<&Option<Box<Tree>>>::new();
        q.push_back(& self.left);
        q.push_back(& self.right);

        loop {
            loop {
                let n = q.pop_front();
                match n {
                    Some(t) => {
                        match t {
                            Some(x) => {
                                print!("{}", x.root);
                                q_next.push_back(& x.left);
                                q_next.push_back(& x.right);
                            },
                            None => {
                                print!("X");
                            }
                        }
                        print!(" ");
                    },
                    None => {
                        break;
                    }
                }
            }
            println!();
            if q_next.len() == 0 {
                break;
            }
            let tmp = q;
            q = q_next;
            q_next =tmp;
        }
        println!()
    }

    fn add(&mut self, x : i64) {
        if x == self.root {
            return
        }

       let target = if x < self.root {
            &mut self.left
        } else {
            &mut self.right
        };

        match target {
             Some(t) => t.add(x),
             None => {
                let newNode = Tree::new(x);
                let boxedNode = Some(Box::new(newNode));
                *target = boxedNode;
            }
        }
    }

    fn find(&self, x : i64) -> bool {
        if x == self.root {
            return true;
        }

       let target = if x < self.root {
            &self.left
        } else {
            &self.right
        };

        match target {
             Some(t) =>  {
               return t.find(x);  
             },
             None => {
                return false;
            }
        }
    }
}

