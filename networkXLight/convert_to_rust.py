def convert_graph_to_rust(input_file, output_file):
    nodes = set()
    edges = []

    with open(input_file, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()

            if not line:
                continue

            parts = line.split()

            if len(parts) < 2:
                continue

            u = parts[0]
            v = parts[1]

            if len(parts) >= 3:
                try:
                    weight = int(float(parts[2]))
                except ValueError:
                    weight = 1
            else:
                weight = 1

            nodes.add(u)
            nodes.add(v)
            edges.append((u, v, weight))

    with open(output_file, "w", encoding="utf-8") as f:
        f.write("use crate::{Graph, AttrValue};\n\n")
        f.write("pub fn build_graph() -> Graph<String> {\n")
        f.write("    let mut g = Graph::<String>::new([\n")
        f.write("        (\n")
        f.write('            "graph".to_string(),\n')
        f.write('            AttrValue::Text("nxGraph".to_string()),\n')
        f.write("        )\n")
        f.write("    ]);\n\n")

        for node in sorted(nodes, key=lambda x: int(x) if x.isdigit() else x):
            f.write(f'    g.add_node("{node}".to_string(), []);\n')

        f.write("\n")

        for u, v, weight in edges:
            f.write("    g.add_edge(\n")
            f.write(f'        "{u}".to_string(),\n')
            f.write(f'        "{v}".to_string(),\n')
            f.write("        [\n")
            f.write(f'            ("weight".to_string(), AttrValue::Int({weight})),\n')
            f.write("        ],\n")
            f.write("    );\n\n")

        f.write("    g\n")
        f.write("}\n")


graph_files = [
    "sparse_graph.txt",
    "medium_graph.txt",
    "dense_graph.txt"
]

for input_file in graph_files:
    output_file = input_file.replace(".txt", ".rs")
    convert_graph_to_rust(input_file, output_file)
    print(f"Rust code written to {output_file}")