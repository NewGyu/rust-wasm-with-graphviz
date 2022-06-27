use graphviz_ffi_example::adaptor::graphviz::{gvz_version, gvz_layout};

fn main() {
    let dot_string = r#"
        digraph G {
            node [shape=rect];

            subgraph cluster_0 {
                style=filled;
                color=lightgrey;
                node [style=filled,color=white];
                a0 -> a1 -> a2 -> a3;
                label = "Hello";
            }

            subgraph cluster_1 {
                node [style=filled];
                b0 -> b1 -> b2 -> b3;
                label = "World";
                color=blue
            }

            start -> a0;
            start -> b0;
            a1 -> b3;
            b2 -> a3;
            a3 -> a0;
            a3 -> end;
            b3 -> end;

            start [shape=Mdiamond];
            end [shape=Msquare];
        }    
    "#;
    println!("Hello, Graphviz! {}", gvz_version());
    println!("{}", gvz_layout(dot_string.to_string()));
}
