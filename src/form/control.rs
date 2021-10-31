#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ControlProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: bool,
}

/// A container with which you can wrap the form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
pub struct Control {
    props: ControlProps,
}

impl Component for Control {
    type Message = ();
    type Properties = ControlProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _ctx:Context,_: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("control");
        classes.push(&self.props.classes);
        if self.props.expanded {
            classes.push("is-expanded");
        }
        let tag = self.props.tag.clone();
        html! {
            <@{tag} class={classes}>
                {self.props.children.clone()}
            </@>
        }
    }
}
