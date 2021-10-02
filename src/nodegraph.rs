use yew::prelude::*;
use std::collections::BTreeMap;
use uuid::Uuid;
use crate::node::*;
use yew::web_sys::{SvgGraphicsElement, Element};
use yew::utils::document;

pub enum NodeGraphMsg {
    /// Add a `Node` the the map of selected nodes.
    SetSelect(Node, MouseEvent),
    /// Drag all slected nodes across the canvas.
    DragSelected(MouseEvent),
}

#[derive(Properties, Clone, PartialEq)]
pub struct NodeGraphProps {
    #[prop_or(600)]
    pub width: u32,

    #[prop_or(400)]
    pub height: u32,
}

pub struct NodeGraph {
    link: ComponentLink<Self>,
    props: NodeGraphProps,
    nodes: BTreeMap<Uuid, Node>,
    selected: BTreeMap<Uuid, Node>,
    svg_ref: NodeRef,
}

impl NodeGraph {
    /// Translate mouse (x, y) coordinates from world space into svg coordinates.
    ///
    /// This function leverages the `svg_ref` reference to the SVG element to
    /// get a translation matrix which is then used to turn the given coordinates into
    /// svg coordinates.
    ///
    /// # Paramters
    ///
    /// * `coords` - 2d mouse coordinates ( MouseEvent.client_x(), MouseEvent.client_y() )
    fn world_to_canvas_2d(&self, coords: Position) -> Option<Position> {
        if let Some(svg_graphics) = self.svg_ref.cast::<SvgGraphicsElement>() {
            if let Some(matrix) = svg_graphics.get_screen_ctm() {
                Some(Position(
                    ((coords.0 as f32 - matrix.e()) / matrix.a()) as i32,
                    ((coords.1 as f32 - matrix.f()) / matrix.d()) as i32
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Component for NodeGraph {
    type Message = NodeGraphMsg;
    type Properties = NodeGraphProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut map = BTreeMap::new();

        let id = Uuid::new_v4();
        map.insert(id.clone(), Node::new(id, "Example Node", Position(50, 50)));

        let id = Uuid::new_v4();
        map.insert(id.clone(), Node::new(id, "Example Node", Position(200, 50)));

        Self {
            link,
            props,
            nodes: map, // All existing nodes. Those are all rendered on the svg canvas.
            selected: BTreeMap::new(), // Container holding all selected child nodes.
            svg_ref: NodeRef::default(), // A reference to the svg canvas element.
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NodeGraphMsg::SetSelect(node, me) => {

                if me.ctrl_key() {
                    if self.selected.contains_key(&node.id) {
                        *node.selected.borrow_mut() = false;
                        self.selected.remove(&node.id);
                    } else {
                        *node.selected.borrow_mut() = true;
                        self.selected.insert(node.id.clone(), node);
                    }
                } else {
                    if !self.selected.contains_key(&node.id) {
                        for (id, n) in self.selected.iter() {
                            *n.selected.borrow_mut() = false;
                        }

                        self.selected.clear();
                        *node.selected.borrow_mut() = true;
                        self.selected.insert(node.id.clone(), node); 
                    }
                }

                let mouse_pos = self.world_to_canvas_2d(Position(
                        me.client_x(), 
                        me.client_y())).unwrap();

                for (id, n) in self.selected.iter_mut() {
                    *n.drag_offset.borrow_mut() = mouse_pos - *n.position.borrow();
                }

                true
            },
            NodeGraphMsg::DragSelected(mouse_event) => {
                let coords = self.world_to_canvas_2d(Position(
                        mouse_event.client_x(), 
                        mouse_event.client_y())).unwrap();

                for (id, n) in self.selected.iter_mut() {
                    *n.position.borrow_mut() = coords - *n.drag_offset.borrow();
                }

                true
            },
            _ => false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let width = format!("{}", self.props.width);
        let height = format!("{}", self.props.height);

        html! {
            <div>
                <ul>
                { for self.selected.iter().map(|(id, n)| html! { <li>{ n.id}</li> }) }
                </ul>
                <svg width=width height=height ref=self.svg_ref.clone()>
                    { for self.nodes.iter().map(|(id, n)| {
                        n.render(self.svg_ref.clone(), 
                            self.link.callback(|n: (Node, MouseEvent)| NodeGraphMsg::SetSelect(n.0, n.1)),
                            self.link.callback(|e: MouseEvent| NodeGraphMsg::DragSelected(e))
                        )
                    })}
                </svg>
            </div>
        }
    }
}
