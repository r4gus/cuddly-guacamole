use yew::prelude::*;
use uuid::Uuid;
use std::rc::Rc;
use std::cell::RefCell;
use yew::web_sys::{SvgGraphicsElement, SvgMatrix};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position(pub i32, pub i32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub id: Uuid,
    pub name: Rc<RefCell<String>>,
    pub position: Rc<RefCell<Position>>,
}

impl Node {
    pub fn new(id: Uuid, name: &str, position: Position) -> Self {
        Self {
            id,
            name: Rc::new(RefCell::new(name.to_string())),
            position: Rc::new(RefCell::new(position)),
        }
    }

    pub fn render(&self, node_ref: NodeRef) -> Html {
        html! {
            <NodeComponent key={self.id.to_string()} node={self.clone()} node_ref=node_ref />
        }
    }
}

pub enum NodeMsg {
    MouseEnter,
    MouseLeave,
    StartDrag(i32, i32),
    Drag(i32, i32),
    EndDrag,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NodeProps {
    node: Node,
    node_ref: NodeRef,
}

pub struct NodeComponent {
    link: ComponentLink<Self>,
    props: NodeProps,
    highlight: bool,
    dragged: bool,
    offset: Position,
}

impl Component for NodeComponent {
    type Message = NodeMsg;
    type Properties = NodeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            highlight: false,
            dragged: false,
            offset: Position(0, 0),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NodeMsg::MouseEnter => {
                self.highlight = true;
                true
            },
            NodeMsg::MouseLeave => {
                self.highlight = false;
                self.dragged = false;
                true
            },
            NodeMsg::StartDrag(x, y) => {
                let mut offset = Position(0, 0);

                if let Some(svg_graphics) = self.props.node_ref.cast::<SvgGraphicsElement>() {
                    if let Some(matrix) = svg_graphics.get_screen_ctm() {
                        // (x, y) in canvas space translates to 
                        // (ax + e, dy + f) in world space, so one needs to
                        // calculate the inverse.
                        offset.0 = ((x as f32 - matrix.e()) / matrix.a()) as i32;
                        offset.1 = ((y as f32 - matrix.f()) / matrix.d()) as i32;
                        
                        offset.0 -= self.props.node.position.borrow().0;
                        offset.1 -= self.props.node.position.borrow().1;

                        self.offset = offset;
                    }
                }                     

                self.dragged = true;
                false
            },
            NodeMsg::EndDrag => {
                self.dragged = false;
                false
            },
            NodeMsg::Drag(x, y) => {
                if self.dragged {
                    let mut pos = self.props.node.position.borrow_mut();
                    if let Some(svg_graphics) = self.props.node_ref.cast::<SvgGraphicsElement>() {
                        if let Some(matrix) = svg_graphics.get_screen_ctm() {
                            // (x, y) in canvas space translates to 
                            // (ax + e, dy + f) in world space, so one needs to
                            // calculate the inverse.
                            pos.0 = ((x as f32 - matrix.e()) / matrix.a()) as i32 - self.offset.0;
                            pos.1 = ((y as f32 - matrix.f()) / matrix.d()) as i32 - self.offset.1;
                        }
                    }                     
                    true
                } else {
                    false
                }
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
        let pos = self.props.node.position.borrow();
        let x = format!("{}", pos.0);
        let y = format!("{}", pos.1);
        let style = format!(
            "fill:blue;stroke:black;stroke-width:5;fill-opacity:0.2;stroke-opacity:{}",
            if self.highlight { 0.6 } else { 0.9 }
            );

        html! {
            <rect x=x y=y rx=10 ry=10 width="50" height="100"
                style=style
                //onmouseenter=self.link.callback(|_| NodeMsg::MouseEnter)
                //onmouseleave=self.link.callback(|_| NodeMsg::MouseLeave)
                //onmousedown=self.link.callback(|e: MouseEvent| NodeMsg::StartDrag(e.client_x(),
                  //                                                                e.client_y()))
                //onmouseup=self.link.callback(|_| NodeMsg::EndDrag)
                //onmousemove=self.link.callback(|e: MouseEvent| NodeMsg::Drag(e.client_x(), 
                 //                                                            e.client_y()))
            />
        }
    }
}


