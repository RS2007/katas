use std::collections::VecDeque;

#[derive(Debug)]
struct TreeNode<'a> {
    value: i32,
    left: Option<&'a TreeNode<'a>>,
    right: Option<&'a TreeNode<'a>>,
}
impl<'a> TreeNode<'a> {
    pub fn to_level_iter(self: &'a TreeNode<'a>) -> TreeNodeIterator<'a> {
        let mut bfsCache: VecDeque<(&'a TreeNode<'a>, usize)> = VecDeque::new();
        bfsCache.push_back((self, 0));
        TreeNodeIterator {
            current: Some(self),
            bfsCache,
        }
    }
}

#[derive(Debug)]
struct TreeNodeIterator<'a> {
    current: Option<&'a TreeNode<'a>>,
    bfsCache: VecDeque<(&'a TreeNode<'a>, usize)>,
}

impl<'a> Iterator for TreeNodeIterator<'a> {
    type Item = (&'a TreeNode<'a>, usize);
    fn next(self: &mut Self) -> Option<<Self as Iterator>::Item> {
        if let Some((currNode, level)) = self.bfsCache.pop_front() {
            if let Some(left) = currNode.left {
                self.bfsCache.push_back((left, level + 1))
            }
            if let Some(right) = currNode.right {
                self.bfsCache.push_back((right, level + 1));
            }
            self.current = Some(currNode);
            return Some((currNode, level));
        }
        self.current = None;
        return None;
    }
}

pub fn main() {
    let c = TreeNode {
        value: 1,
        left: None,
        right: None,
    };
    let b = TreeNode {
        value: 2,
        left: None,
        right: None,
    };
    let a = TreeNode {
        value: 3,
        left: Some(&b),
        right: Some(&c),
    };
    let mut arrList: Vec<Vec<i32>> = vec![vec![]];
    let mut lastLevel: usize = 0;
    for val in a.to_level_iter() {
        if val.1 == lastLevel {
            (&mut arrList).last_mut().unwrap().push(val.0.value);
            continue;
        } else {
            let mut newArrList: Vec<i32> = vec![];
            newArrList.push(val.0.value);
            lastLevel = val.1;
            (&mut arrList).push(newArrList);
        }
    }
    println!("{:?}", arrList);
}
