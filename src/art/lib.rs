// in future we use preallcoated nodes

pub struct ArtTree {
    root: Option<Box<dyn Node>>,
}

impl ArtTree {
    // insert, delete, update, search
    pub fn insert(&mut self, key: &[u8], value: i32) {
        ArtTree::insert0(&mut self.root, key, value, 0)
    }

    fn insert0(node: &mut Option<Box<dyn Node>>, key: &[u8], value: i32, depth: usize) {
        match node {
            Some(ref mut boxed) => {
                if boxed.is_leaf() { // node == leaf
                    let new_box = Box::new(Node4::new());
                    // add leaf to new_box
                    // add value that we want to insert into new_box
                    let leaf = std::mem::replace(node, Some(new_box));
                    return;
                }
                let next = ArtTree::find_child(&boxed, key[depth]); //  node == Node
                match next {
                    Some(_) => {
                        ArtTree::insert0(next, key, value, depth + 1);
                    },
                    None => {
                        if boxed.is_full() {
                            let tmp = boxed.grow();
                            node.replace(tmp);
                            // add_child
                        }
                    }
                }
            },
            None => { // node == null pointer
                let new_leaf: Box<dyn Node> = Box::new(Leaf::new(value));
                *node = Some(new_leaf);
            }
        }
    }

    fn add_child(node: &mut Option<Box<dyn Node>>, byte: u8, node_to_insert: Option<Box<dyn Node>>) {

    }

    pub fn delete(&self, key: &[u8]) -> bool {
        true
    }

    pub fn update(&self, key: &[u8], value: i32) -> bool {
        true
    }

    pub fn search(&self, key: &[u8]) -> Option<&i32> {
        self.search0(&self.root, key, 0)
    }

    fn leaf_match(&self, node: &Box<dyn Node>, key: &[u8], depth: usize) -> bool {
        true
    }

    fn check_prefix(&self, node: &Box<dyn Node>, key: &[u8], depth: usize) -> usize {
        0
    }

    fn search0<'a>(&'a self, node: &'a Option<Box<dyn Node>>, key: &[u8], mut depth: usize) -> Option<&'a i32> {
        match node {
            Some(node) => {
                if node.is_leaf() {
                    if self.leaf_match(&node, &key, depth) {
                        return Some(node.get_value());
                    }
                    return None;
                }

                if self.check_prefix(&node, &key, depth) != node.prefix_len() {
                    return None;
                }

                // depth += prefixLength
                let next = ArtTree::find_child(node, key[depth]);
                self.search0(next, key, depth + 1)
            },
            None => {
                None
            }
        }
    }

    fn find_child<'a>(node: &'a Box<dyn Node>, byte: u8) -> &'a Option<Box<dyn Node>> {
        node.find_child(byte)
    }
}

trait Node {
    fn is_leaf(&self) -> bool;
    fn prefix_len(&self) -> usize;
    fn get_value(&self) -> &i32;
    fn find_child(&self, byte: u8) -> &Option<Box<dyn Node>>;
    fn is_full(&self) -> bool;
    fn grow(&mut self) -> Box<dyn Node>;
    fn add_child(&mut self, byte: u8, node_to_add: Option<Box<dyn Node>>);
}

struct Node4 {
    pub key: [u8; 4],
    pub prefix_len: usize,
    pub child_ptr: [Option<Box<dyn Node>>; 4]
}

impl Node4 {
    fn new() -> Self {
        Node4 {
            key: [0; 4],
            prefix_len: 0,
            child_ptr: [(); 4].map(|_| None)
        }
    }
}

impl Node for Node4 {
    fn is_leaf(&self) -> bool {
        false
    }
    fn prefix_len(&self) -> usize {
        self.prefix_len
    }
    fn get_value(&self) -> &i32 {
        panic!("Non-leaf should not get_value");
    }

    fn find_child(&self, byte: u8) -> &Option<Box<dyn Node>> {
        for i in 0..4usize {
            match self.key[i] == byte {
                true => {
                    return &self.child_ptr[i];
                }
                _ => {}
            }
        }
        &None
    }

    fn is_full(&self) -> bool {
        for opt in self.child_ptr.iter() {
            match opt {
                Some(boxed) => {
                    return true;
                },
                None => {}
            }
        }
        false
    }

    fn grow(&mut self) -> Box<dyn Node> {
        let mut new = Box::new(Node16::new());
        for (i, item) in &mut self.key.iter().enumerate() {
            new.key[i] = *item;
            new.child_ptr[i] = self.child_ptr[i].take();
        }
        new
    }
    // iterate in reverse from i-2 with assumption that Node is not Full
    fn add_child(&mut self, mut byte: u8, mut node_to_add: Option<Box<dyn Node>>) {
        for i in (0..3).rev() {
            if self.child_ptr[i].is_some()  { // right is None, curr is not None
                if (self.key[i] < byte) { // no right shift
                    self.key[i + 1] = byte; // put byte on rightmost None
                    self.child_ptr[i + 1] = node_to_add.take();
                    break;
                }
                else { // right shift
                    self.key[i + 1] = self.key[i];
                    self.child_ptr[i + 1] = self.child_ptr[i].take();
                    if i == 0 {
                        self.key[i] = byte;
                        self.child_ptr[i] = node_to_add.take();
                    }
                }
            }
        }
    }
}

// We can use SIMD
struct Node16 {
    key: [u8; 16],
    prefix_len: usize,
    child_ptr: [Option<Box<dyn Node>>; 16]
}

impl Node16 {
    fn new() -> Self {
        Node16 {
            key: [0; 16],
            prefix_len: 0,
            child_ptr: [(); 16].map(|_| None),
            //child_ptr: [None; 16]
        }
    }
}

impl Node for Node16 {
    fn is_leaf(&self) -> bool {
        false
    }
    fn prefix_len(&self) -> usize {
        self.prefix_len
    }
    fn get_value(&self) -> &i32 {
        panic!("Non-leaf should not get_value");
    }

    fn find_child(&self, byte: u8) -> &Option<Box<dyn Node>> {
        &None
    }

    fn is_full(&self) -> bool {
        todo!()
    }

    fn grow(&mut self) -> Box<dyn Node> {
        todo!()
    }

    fn add_child(&mut self, byte: u8, node_to_add: Option<Box<dyn Node>>) {
        todo!()
    }
}

struct Node48 {
    key: [u8; 256], // values are indices to the child_ptr
    prefix_len: usize,
    child_ptr: [Box<dyn Node>; 48]
}

impl Node for Node48 {
    fn is_leaf(&self) -> bool {
        false
    }
    fn prefix_len(&self) -> usize {
        self.prefix_len
    }
    fn get_value(&self) -> &i32 {
        panic!("Non-leaf should not get_value");
    }

    fn find_child(&self, byte: u8) -> &Option<Box<dyn Node>> {
        &None
    }

    fn is_full(&self) -> bool {
        todo!()
    }

    fn grow(&mut self) -> Box<dyn Node> {
        todo!()
    }

    fn add_child(&mut self, byte: u8, node_to_add: Option<Box<dyn Node>>) {
        todo!()
    }
}

struct Node256 {
    child_ptr: [Box<dyn Node>; 256],
    prefix_len: usize,
}

impl Node for Node256 {
    fn is_leaf(&self) -> bool {
        false
    }
    fn prefix_len(&self) -> usize {
        self.prefix_len
    }
    fn get_value(&self) -> &i32 {
        panic!("Non-leaf should not get_value");
    }

    fn find_child(&self, byte: u8) -> &Option<Box<dyn Node>> {
        &None
    }

    fn is_full(&self) -> bool {
        todo!()
    }

    fn grow(&mut self) -> Box<dyn Node> {
        todo!()
    }

    fn add_child(&mut self, byte: u8, node_to_add: Option<Box<dyn Node>>) {
        todo!()
    }
}

struct Leaf {
    value: i32
}

impl Leaf {
    fn new(value: i32) -> Self {
        Leaf { value }
    }
}

impl Node for Leaf {

    fn is_leaf(&self) -> bool {
        true
    }
    fn prefix_len(&self) -> usize {
        return 0;
    }
    fn get_value(&self) -> &i32 {
        &self.value
    }

    fn find_child(&self, byte: u8) -> &Option<Box<dyn Node>> {
        panic!("Non-leaf should not get_value");
    }

    fn is_full(&self) -> bool {
        todo!()
    }

    fn grow(&mut self) -> Box<dyn Node> {
        todo!()
    }

    fn add_child(&mut self, byte: u8, node_to_add: Option<Box<dyn Node>>) {
        todo!()
    }
}



// fn insert(node: Option<Box<Node>>, key: u8, leaf: Node, depth: usize) {
//     match node {
//         Some(node) => {
//             if is_leaf(node) {
//                 new_node = make_node4();
//                 key2 = load_key(node);
//             }
//         },
//         None => {
//             replace(node, leaf);
//             return;
//         }
//     }
//         if isLeaf(node) // expand node
//     newNode = makeNode4()
//     key2 = loadKey(node)
//     for (i = depth; key[i] == key2[i]; i = i + 1)
//     newNode.prefix[i - depth] = key[i]
//     newNode.prefixLen = i - depth
//     depth = depth + newNode.prefixLen
//     addChild(newNode, key[depth], leaf)
//     addChild(newNode, key2[depth], node)
//     replace(node, newNode)
//     return
//         p = checkPrefix(node, key, depth)
//     if p != node.prefixLen // prefix mismatch
//     newNode = makeNode4()
//     addChild(newNode, key[depth + p], leaf)
//     addChild(newNode, node.prefix[p], node)
//     newNode.prefixLen = p
//     memcpy(newNode.prefix, node.prefix, p)
//     node.prefixLen = node.prefixLen - (p + 1)
//     memmove(node.prefix, node.prefix + p + 1, node.prefixLen)
//     replace(node, newNode)
//     return
//         depth = depth + node.prefixLen
//     next = findChild(node, key[depth])
//     if next // recurse
//     insert(next, key, leaf, depth + 1)
//     else // add to inner node
//     if isFull(node)
//     grow(node)
//     addChild(node, key[depth], leaf)
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }

    #[test]
    fn test_mut() {
        let var = Some(Box::new(Node4::new()));
    }
    #[test]
    fn test_node4_add_child() {
        // let mut node4 = Node4::new();
        //
        // for i in (1..5) {
        //     node4.add_child(i, Some(Box::new(Node4::new())));
        //     let mut max_key = 0;
        //     for j in (0..i) {
        //         assert!(node4.key[j] > max_key);
        //         assert!(node4.child_ptr[j].is_okay());
        //         max_key = node4.key[j];
        //     }
        //     for j in (i..4) {
        //         assert!(node4.child_ptr[j].is_none());
        //     }
        // }
    }
}
