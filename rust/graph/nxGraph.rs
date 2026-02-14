use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum AttrValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}

// Simple attribute map like your Python `dict`
pub type AttrMap = HashMap<String, AttrValue>;

// A direct, beginner-friendly Graph that mirrors your Python layout:
// - graph: graph-level attributes
// - node: node -> attributes
// - adj_outer: from_node -> (to_node -> edge attributes)

#[derive(Debug, Clone)]
pub struct Graph<N>
where 
    N: Eq + Hash + Clone
{
    pub graph: AttrMap,                                
    pub node: HashMap<N, AttrMap>,
    pub adj_outer: HashMap<N, HashMap<N, AttrMap>>,     // <- Adj_outer = { source: { dest: {Edge} } }
}

impl<N> Graph<N>
where
    N: Eq + Hash + Clone
{
    pub fn new<I>(graph_attr: I) -> Self 
    where 
        I: IntoIterator<Item = (String, AttrValue)>,
    {
        let mut g = Graph {
            graph: HashMap::new(),
            node: HashMap::new(),
            adj_outer: HashMap::new(),
        };

        g.graph.extend(graph_attr);
        g
    }

    pub fn add_node<I>(&mut self, new_node: N, attr: I)
    where
        I: IntoIterator<Item = (String, AttrValue)>,
    {
        if !self.node.contains_key(&new_node) {
            self.adj_outer.insert(new_node.clone(), HashMap::new());
            
            let mut attr_dict = AttrMap::new();
            attr_dict.extend(attr);
            self.node.insert(new_node, attr_dict);
        } else {
            if let Some(existing) = self.node.get_mut(&new_node) {
                existing.extend(attr);
            }
        }
    } 

    pub fn add_edge<I>(&mut self, src: N, dst: N, attr: I)
    where
        I: IntoIterator<Item = (String, AttrValue)> + Clone,
    {
        self.node.entry(src.clone()).or_default();
        self.node.entry(dst.clone()).or_default();

        self.adj_outer.entry(src.clone()).or_default();
        self.adj_outer.entry(dst.clone()).or_default();
        
        {
            let nbrs = self.adj_outer.get_mut(&src).unwrap();
            let nbr_dict = nbrs.entry(dst.clone()).or_default();
            nbr_dict.extend(attr.clone())
        }

        {
            let nbrs = self.adj_outer.get_mut(&dst).unwrap();
            let nbr_dict = nbrs.entry(src.clone()).or_default();
            nbr_dict.extend(attr)
        }
    }
}


fn main() {
    let mut g = Graph::<String>::new([
        (
            "name".to_string(),
            AttrValue::Text("nxGraph".to_string()),
        )
    ]);

    g.add_node("A".to_string(), [("color".to_string(), AttrValue::Text("amber".to_string()))]);
    g.add_node("B".to_string(), [("color".to_string(), AttrValue::Bool(true))]);
    g.add_edge(
        "A".to_string(),
        "B".to_string(),
        [("weight".to_string(),  AttrValue::Int(5))],
    );
    
    println!("{:#?}", g);

}