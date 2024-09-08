crate::ix!();

pub fn update_bc_for_graph<GH>(
    scores:      &mut BetweennessScores,
    graph:       &Graph<GH>,
    comp_type:   CompType,
    mut edge:    Edge,
    num_threads: usize,
    op:          Operation)
-> Result<(), BetweennessCentralityError>
where Graph<GH>: MappedNodes + GetNodeIdRange + FindEdgeBccWithComponent<GH>
{
    let mut component = Component::new_from_graph_ref(graph, "update_bc_for_graph::component");

    let mut delta_bc_of_vertices = graph.create_scores_vector();

    icentral(
        num_threads,
        &mut delta_bc_of_vertices, 
        arcmut![component],
        edge, 
        Some(op)
    )?;

    for i in 0..delta_bc_of_vertices.len() {

        let id = NodeId::from(i);

        scores.increase_score_for_node(
            id, 
            delta_bc_of_vertices.score_for_node(id)
        );
    }

    Ok(())
}

pub fn execute_biconnected_component_update_operation<GH>(
    graph:     &mut Graph<GH>, 
    component: &mut Component,
    edge:      &mut Edge,
    op:        &Operation
) 
where 
    Graph<GH>
    : RemoveEdge
    + FindEdgeBccWithComponent<GH>,

    GH
    : ConnectedComponentSize
    + MappedNodes
    + NumNodes
    + BccGraphHashInterface
    + GetEdges
    + HasEdge
    + RemoveEdge
    + GetNeighborsForNode
    + GetNodeIdRange
    + Named
    + CreateNamedEmpty
{
    match op {

        Operation::Insertion => {

            // IMP: assumes @edge is not in
            // @graph, @edge will not be in @component
            //
            graph.find_edge_bcc_with_component(component, edge);
        }

        Operation::Deletion => {

            // IMP: assumes @edge is not in
            // @graph, @edge will not be in @component
            //
            //      (remove @edge from @graph)
            graph.remove_edge(&edge);
            graph.find_edge_bcc_with_component(component, edge);
            graph.insert_edge(&edge);
        }
    }

    // DEBUG
    //
    // printf(
    //   "BiconnectedComponents # vertices [%d], # edges [%d] ", 
    //   component.subgraph.len(), 
    //   component.subgraph.num_edges()
    // );
    //
    // map @edge to the new edge

    edge.src = component.label_map_outin(edge.src);
    edge.dst = component.label_map_outin(edge.dst);

    if op.is_deletion() {
        component.insert_edge_between_nodes(edge.src, edge.dst);
    }
}

pub fn update_bc_for_biconnected_component<GH>(
    scores:      &mut BetweennessScores,
    graph:       &mut Graph<GH>,
    comp_type:   CompType,
    mut edge:    Edge,
    num_threads: usize,
    op:          Operation)
-> Result<(), BetweennessCentralityError>
where GH
: ConnectedComponentSize
+ MappedNodes
+ NumNodes
+ GetEdges
+ HasEdge
+ RemoveEdge
+ GetNeighborsForNode
+ GetNodeIdRange
+ Named
+ CreateNamedEmpty
+ BccGraphHashInterface
{
    let mut component = Component::new_biconnected("update_bc_for_biconnected_component::component");

    execute_biconnected_component_update_operation(
        graph,
        &mut component,
        &mut edge,
        &op
    );

    let mut delta_bc_of_vertices = graph.create_scores_vector();

    let component = arcmut![component];

    icentral(
        num_threads,
        &mut delta_bc_of_vertices, 
        component.clone(), 
        edge, 
        Some(op)
    )?;

    if let component = component.lock()? {

        for node in delta_bc_of_vertices.nodeid_range() {

            let actual_node_id: NodeId = component.label_map_inout(node);

            scores.increase_score_for_node(
                actual_node_id, 
                delta_bc_of_vertices.score_for_node(node)
            );
        }
    }

    Ok(())
}

/**
  | For edge additions, the edge must not
  | be in the graph
  | 
  | For edge deletions, the edge must not
  | be a bridge
  |
  | Updates the BC values in @scores in place
  | for nodes in @graph using @comp_type
  | decomposition
  | 
  | TODO: should handle all kinds of decompositions
  | (graph/BiconnectedComponents/MinimumUnionCycle)
  | 
  | IMP: assumes @scores is the right size
  | and has BC values
  |
  */
pub fn update_bc<GH>(
    scores:      &mut BetweennessScores,
    graph:       &mut Graph<GH>,
    comp_type:   CompType,
    mut edge:    Edge,
    num_threads: Option<usize>,
    op:          Option<Operation>)
-> Result<(), BetweennessCentralityError>
where Graph<GH>
: MappedNodes
+ GetNodeIdRange
, GH
: HasEdge
+ ConnectedComponentSize
+ CreateNamedEmpty
+ GetEdges
+ GetNeighborsForNode
+ GetNodeIdRange
+ InsertEdge
+ MappedNodes
+ NumEdges
+ NumNodes
+ RemoveEdge
+ Named
{
    let num_threads:     usize = num_threads.unwrap_or(1);
    let op:          Operation = op.unwrap_or(Operation::Insertion);

    match comp_type {

        CompType::Graph => update_bc_for_graph(
            scores,
            graph,
            comp_type,
            edge,
            num_threads,
            op
        )?,

        CompType::BiconnectedComponent => update_bc_for_biconnected_component(
            scores,
            graph,
            comp_type,
            edge,
            num_threads,
            op
        )?,

        CompType::MinimumUnionCycle => {
            unimplemented!();
        }
    }

    Ok(())
}
