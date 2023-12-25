#[cfg(test)]
mod tests {
    use std::cmp::{max, min};
    use indoc::indoc;
    use crate::input_reader::{read_input_file, read_lines};
    use graphrs::{algorithms::community::louvain, Edge, Graph, GraphSpecs};
    use itertools::Itertools;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day25.txt");

        let (group1, group2) = find_groups(input);
        assert_eq!(580800, min(group1.len(), group2.len()) *  max(group1.len(), group2.len()));
    }

    #[test]
    fn it_finds_groups() {
        let input = indoc! {"
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
        "};

        let (group1, group2) = find_groups(input);
        assert_eq!(6, min(group1.len(), group2.len()));
        assert_eq!(9, max(group1.len(), group2.len()));
    }

    fn find_groups(input: &str) -> (Vec<String>, Vec<String>) {
        let mut edges: Vec<Edge<String, _>> = vec![];

        for line in read_lines(input) {
            let parts = line.split(": ").collect::<Vec<_>>();
            let node = parts[0];
            parts[1].split(" ").for_each(|n| {
                edges.push(Edge::new(node.to_string(), n.to_string()));
                edges.push(Edge::new(n.to_string(), node.to_string()));
            });
        }

        let graph: Graph<String, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let partitions = louvain::louvain_partitions(&graph, false, Some(0f64), Some(4f64), None).unwrap();
        let best_partition = partitions.last().unwrap();

        let group1: Vec<String> = best_partition
            .first()
            .unwrap()
            .iter()
            .cloned()
            .sorted()
            .collect();

        let group2: Vec<String> = best_partition
            .last()
            .unwrap()
            .iter()
            .cloned()
            .sorted()
            .collect();
        (group1, group2)
    }
}