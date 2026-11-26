//! Motor puro en Rust de layout jerárquico top-down (sin colisiones) para grafos AST en SVG.

use super::ast_trace::AstNodeInfo;
use std::collections::{HashMap, HashSet};

/// Ancho fijo de cada caja de nodo AST en unidades SVG.
pub const NODE_WIDTH: f64 = 156.0;
/// Alto fijo de cada caja de nodo AST en unidades SVG.
pub const NODE_HEIGHT: f64 = 52.0;
/// Separación horizontal mínima entre subárboles vecinos.
pub const H_GAP: f64 = 24.0;
/// Separación vertical entre niveles (deja aire para badges de resultado y etiquetas de arista).
pub const V_GAP: f64 = 66.0;
/// Margen horizontal exterior del lienzo SVG.
pub const CANVAS_PAD_X: f64 = 48.0;
/// Margen vertical exterior del lienzo SVG.
pub const CANVAS_PAD_Y: f64 = 44.0;
/// Desplazamiento vertical del bus ortogonal desde el borde inferior del nodo padre.
const ELBOW_DROP_Y: f64 = 36.0;

/// Geometría posicionada de un nodo AST dentro del espacio de coordenadas SVG.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutNode {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub cx: f64,
    pub cy: f64,
}

/// Conector ortogonal padre -> hijo junto con la posición de su etiqueta de arista.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutEdge {
    pub parent_id: usize,
    pub child_id: usize,
    pub label: String,
    pub path_d: String,
    pub label_x: f64,
    pub label_y: f64,
}

/// Resultado completo del layout geométrico del AST listo para renderizar en SVG.
#[derive(Debug, Clone, PartialEq)]
pub struct AstLayout {
    pub nodes: Vec<LayoutNode>,
    pub edges: Vec<LayoutEdge>,
    pub view_width: f64,
    pub view_height: f64,
}

impl AstLayout {
    /// Busca la geometría posicionada de un nodo por su identificador.
    pub fn node(&self, node_id: usize) -> Option<&LayoutNode> {
        if let Some(candidate) = self.nodes.get(node_id) {
            if candidate.id == node_id {
                return Some(candidate);
            }
        }
        self.nodes.iter().find(|item| item.id == node_id)
    }
}

/// Calcula posiciones `(x, y)` sin solapamientos y conectores ortogonales para todo el AST.
///
/// Utiliza asignación recursiva de ancho de subárbol estilo Reingold-Tilford/Walker centrando
/// cada padre sobre el punto medio entre su primer y último hijo. Por invariante de intervalos
/// disjuntos, ninguna pareja de nodos puede superponerse horizontal ni verticalmente.
pub fn compute_ast_layout(nodes: &[AstNodeInfo], root_id: usize) -> AstLayout {
    if nodes.is_empty() {
        return AstLayout {
            nodes: Vec::new(),
            edges: Vec::new(),
            view_width: NODE_WIDTH + CANVAS_PAD_X * 2.0,
            view_height: NODE_HEIGHT + CANVAS_PAD_Y * 2.0,
        };
    }

    let by_id: HashMap<usize, &AstNodeInfo> = nodes.iter().map(|node| (node.id, node)).collect();
    let effective_root = if by_id.contains_key(&root_id) {
        root_id
    } else {
        nodes[0].id
    };

    let mut subtree_widths: HashMap<usize, f64> = HashMap::with_capacity(nodes.len());
    let mut measure_visited: HashSet<usize> = HashSet::with_capacity(nodes.len());
    measure_subtree(
        effective_root,
        &by_id,
        &mut measure_visited,
        &mut subtree_widths,
    );

    let mut placed_map: HashMap<usize, LayoutNode> = HashMap::with_capacity(nodes.len());
    let mut place_visited: HashSet<usize> = HashSet::with_capacity(nodes.len());
    place_subtree(
        effective_root,
        CANVAS_PAD_X,
        0,
        &by_id,
        &subtree_widths,
        &mut place_visited,
        &mut placed_map,
    );

    // Conservar orden determinista según `nodes` de entrada
    let mut layout_nodes = Vec::with_capacity(placed_map.len());
    let mut max_right = NODE_WIDTH + CANVAS_PAD_X;
    let mut max_bottom = NODE_HEIGHT + CANVAS_PAD_Y;

    for info in nodes {
        if let Some(pos) = placed_map.get(&info.id) {
            max_right = max_right.max(pos.x + pos.width);
            max_bottom = max_bottom.max(pos.y + pos.height);
            layout_nodes.push(pos.clone());
        }
    }

    // Generar aristas ortogonales `M x1 y1 V ym H x2 V y2`
    let mut layout_edges = Vec::with_capacity(layout_nodes.len());
    for info in nodes {
        let Some(parent_pos) = placed_map.get(&info.id) else {
            continue;
        };
        for &child_id in &info.children {
            let Some(child_pos) = placed_map.get(&child_id) else {
                continue;
            };
            let x1 = parent_pos.cx;
            let y1 = parent_pos.y + parent_pos.height;
            let x2 = child_pos.cx;
            let y2 = child_pos.y;
            let ym = y1 + ELBOW_DROP_Y;

            let path_d = if (x1 - x2).abs() < 0.5 {
                format!("M {x1:.1} {y1:.1} V {y2:.1}")
            } else {
                format!("M {x1:.1} {y1:.1} V {ym:.1} H {x2:.1} V {y2:.1}")
            };

            let label = by_id
                .get(&child_id)
                .map(|child| child.edge.clone())
                .unwrap_or_default();

            layout_edges.push(LayoutEdge {
                parent_id: info.id,
                child_id,
                label,
                path_d,
                label_x: x2,
                label_y: y2 - 10.0,
            });
        }
    }

    // Espacio extra inferior para badges de valor resuelto en nodos hoja
    let view_width = max_right + CANVAS_PAD_X;
    let view_height = max_bottom + CANVAS_PAD_Y + 24.0;

    AstLayout {
        nodes: layout_nodes,
        edges: layout_edges,
        view_width,
        view_height,
    }
}

fn measure_subtree(
    node_id: usize,
    by_id: &HashMap<usize, &AstNodeInfo>,
    visited: &mut HashSet<usize>,
    widths: &mut HashMap<usize, f64>,
) -> f64 {
    if !visited.insert(node_id) {
        return *widths.get(&node_id).unwrap_or(&NODE_WIDTH);
    }

    let Some(info) = by_id.get(&node_id) else {
        widths.insert(node_id, NODE_WIDTH);
        return NODE_WIDTH;
    };

    let valid_children: Vec<usize> = info
        .children
        .iter()
        .copied()
        .filter(|cid| by_id.contains_key(cid) && !visited.contains(cid))
        .collect();

    if valid_children.is_empty() {
        widths.insert(node_id, NODE_WIDTH);
        return NODE_WIDTH;
    }

    let mut total_children_w = 0.0;
    for (idx, child_id) in valid_children.iter().enumerate() {
        if idx > 0 {
            total_children_w += H_GAP;
        }
        total_children_w += measure_subtree(*child_id, by_id, visited, widths);
    }

    let width = NODE_WIDTH.max(total_children_w);
    widths.insert(node_id, width);
    width
}

fn place_subtree(
    node_id: usize,
    left_x: f64,
    depth: usize,
    by_id: &HashMap<usize, &AstNodeInfo>,
    widths: &HashMap<usize, f64>,
    visited: &mut HashSet<usize>,
    placed: &mut HashMap<usize, LayoutNode>,
) -> f64 {
    if !visited.insert(node_id) {
        return placed
            .get(&node_id)
            .map_or(left_x + NODE_WIDTH / 2.0, |n| n.cx);
    }

    let y = CANVAS_PAD_Y + (depth as f64) * (NODE_HEIGHT + V_GAP);
    let my_width = *widths.get(&node_id).unwrap_or(&NODE_WIDTH);

    let valid_children: Vec<usize> = by_id
        .get(&node_id)
        .map(|info| {
            info.children
                .iter()
                .copied()
                .filter(|cid| by_id.contains_key(cid) && !visited.contains(cid))
                .collect()
        })
        .unwrap_or_default();

    let cx = if valid_children.is_empty() {
        left_x + my_width / 2.0
    } else {
        let mut children_span = 0.0;
        for (idx, child_id) in valid_children.iter().enumerate() {
            if idx > 0 {
                children_span += H_GAP;
            }
            children_span += *widths.get(child_id).unwrap_or(&NODE_WIDTH);
        }

        let mut cursor_x = left_x + (my_width - children_span) / 2.0;
        let mut first_cx = 0.0;
        let mut last_cx = 0.0;

        for (idx, child_id) in valid_children.iter().enumerate() {
            let child_w = *widths.get(child_id).unwrap_or(&NODE_WIDTH);
            let child_cx = place_subtree(
                *child_id,
                cursor_x,
                depth + 1,
                by_id,
                widths,
                visited,
                placed,
            );
            if idx == 0 {
                first_cx = child_cx;
            }
            last_cx = child_cx;
            cursor_x += child_w + H_GAP;
        }

        (first_cx + last_cx) / 2.0
    };

    let x = cx - NODE_WIDTH / 2.0;
    let cy = y + NODE_HEIGHT / 2.0;

    placed.insert(
        node_id,
        LayoutNode {
            id: node_id,
            x,
            y,
            width: NODE_WIDTH,
            height: NODE_HEIGHT,
            cx,
            cy,
        },
    );

    cx
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_node(
        id: usize,
        parent_id: Option<usize>,
        edge: &str,
        children: Vec<usize>,
    ) -> AstNodeInfo {
        AstNodeInfo {
            id,
            parent_id,
            edge: edge.to_string(),
            kind: "Node".to_string(),
            label: format!("n{id}"),
            detail: String::new(),
            children,
        }
    }

    fn assert_no_overlaps(layout: &AstLayout) {
        for i in 0..layout.nodes.len() {
            for j in (i + 1)..layout.nodes.len() {
                let a = &layout.nodes[i];
                let b = &layout.nodes[j];
                let x_overlap = a.x < (b.x + b.width) && (a.x + a.width) > b.x;
                let y_overlap = a.y < (b.y + b.height) && (a.y + a.height) > b.y;
                assert!(
                    !(x_overlap && y_overlap),
                    "Solapamiento detectado entre nodo {} ({:?}) y nodo {} ({:?})",
                    a.id,
                    a,
                    b.id,
                    b
                );
            }
        }
    }

    #[test]
    fn layout_unbalanced_tree_has_zero_overlaps() {
        let nodes = vec![
            mk_node(0, None, "", vec![1, 2, 3]),
            mk_node(1, Some(0), "stmt[0]", vec![4, 5]),
            mk_node(2, Some(0), "stmt[1]", vec![]),
            mk_node(3, Some(0), "stmt[2]", vec![6, 7, 8]),
            mk_node(4, Some(1), "left", vec![9, 10]),
            mk_node(5, Some(1), "right", vec![]),
            mk_node(6, Some(3), "cond", vec![]),
            mk_node(7, Some(3), "then", vec![11]),
            mk_node(8, Some(3), "else", vec![12, 13]),
            mk_node(9, Some(4), "left", vec![]),
            mk_node(10, Some(4), "right", vec![]),
            mk_node(11, Some(7), "stmt[0]", vec![]),
            mk_node(12, Some(8), "left", vec![]),
            mk_node(13, Some(8), "right", vec![]),
        ];

        let layout = compute_ast_layout(&nodes, 0);
        assert_eq!(layout.nodes.len(), nodes.len());
        assert_eq!(layout.edges.len(), nodes.len() - 1);
        assert_no_overlaps(&layout);

        for node in &layout.nodes {
            assert!(node.x >= 0.0);
            assert!(node.y >= 0.0);
            assert!(node.x + node.width <= layout.view_width);
            assert!(node.y + node.height <= layout.view_height);
        }
    }

    #[test]
    fn layout_single_root_fits_viewbox() {
        let nodes = vec![mk_node(0, None, "", vec![])];
        let layout = compute_ast_layout(&nodes, 0);
        assert_eq!(layout.nodes.len(), 1);
        assert!(layout.edges.is_empty());
        assert_no_overlaps(&layout);
    }
}
