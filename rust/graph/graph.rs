use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum AttrValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}

pub type AttrMap = HashMap<String, AttrValue>;

#[derive(Debug, Clone)]
pub struct Edge {
    pub to: usize,
    pub weight: u32,
    pub attr: AttrMap,
}

#[derive(Debug)]
pub struct Graph<K>
where
    K: Eq + Hash + Clone,
{
    pub graph: AttrMap,                 // graph{attr}
    pub node: Vec<AttrMap>,             // index = ID, [{K:V, K:V}, {Index 1 K:V Pairs}]
    pub adj: Vec<Vec<Edge>>,            // 

    // Mapping variables to ints
    pub key_to_id: HashMap<K, usize>,   // {Key: ID}
    pub id_to_key: Vec<K>,              // id = index, index -> Key
}

impl<K> Graph<K> 
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            node:  Vec::new(),
            adj:   Vec::new(),
            key_to_id: HashMap::new(),
            id_to_key: Vec::new(),
        }
    }

    fn get_id(&mut self, key: K) -> usize {
        if let Some(&id) = self.key_to_id.get(&key) {
            return id;
        }

        let id = self.id_to_key.len();
        self.key_to_id.insert(key.clone(), id);
        self.id_to_key.push(key);
        self.node.push(HashMap::new());
        self.adj.push(Vec::new());
        id
    }

    pub fn add_node(&mut self, key: K, attrs: AttrMap) -> usize {
        let id = self.get_id(key);
        self.node[id].extend(attrs);
        id
    }

}

fn main() {
    println!("Hello, world!");
}