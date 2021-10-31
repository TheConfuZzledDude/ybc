#![allow(clippy::redundant_closure_call)]
use yew::events::MouseEvent;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DeleteProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "button".into())]
    pub tag: String,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// A versatile delete cross.
///
/// [https://bulma.io/documentation/elements/delete/](https://bulma.io/documentation/elements/delete/)
pub struct Delete {
    props: DeleteProps,
}

impl Component for Delete {
    type Message = ();
    type Properties = DeleteProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, ctx: Context, _: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("delete");
        classes.push(&self.props.classes);
        let tag = self.props.tag.clone();
        html! {
            <@{tag} class={classes} onclick={self.props.onclick.clone()}>
                {self.props.children.clone()}
            </@>
        }
    }
}
