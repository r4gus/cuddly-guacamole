use yew::prelude::*;
use std::collections::BTreeMap;
use uuid::Uuid;
use crate::node::*;
use yew::web_sys::{SvgGraphicsElement, Element};
use yew::utils::document;

pub enum CanvasMsg {

}

#[derive(Properties, Clone, PartialEq)]
pub struct CanvasProps {
    #[prop_or_default]
    pub children: Children, 

    #[prop_or(600)]
    pub width: u32,

    #[prop_or(400)]
    pub height: u32,
}

pub struct Canvas {
    link: ComponentLink<Self>,
    props: CanvasProps,
    nodes: BTreeMap<Uuid, Node>,
    selected: BTreeMap<Uuid, Node>,
}

impl Component for Canvas {
    type Message = CanvasMsg;
    type Properties = CanvasProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut map = BTreeMap::new();

        let id = Uuid::new_v4();
        map.insert(id.clone(), Node::new(id, "Example Node", Position(50, 50)));

        Self {
            link,
            props,
            nodes: map,
            selected: BTreeMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
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
        let svg_ref: NodeRef = NodeRef::default();

        html! {
            <svg width=width height=height ref=svg_ref.clone()>
                { for self.nodes.iter().map(|(id, n)| n.render(svg_ref.clone())) }
            </svg>
        }
    }
}
