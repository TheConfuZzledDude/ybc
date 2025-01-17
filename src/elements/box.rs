use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BoxProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A white box to contain other elements.
///
/// [https://bulma.io/documentation/elements/box/](https://bulma.io/documentation/elements/box/)
pub struct Box {
    props: BoxProps,
}

impl Component for Box {
    type Message = ();
    type Properties = BoxProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("box");
        classes.push(&self.props.classes);
        html! {
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}
