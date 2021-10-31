use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Add a `32px` margin to the left and right sides of the container.
    #[prop_or_default]
    pub fluid: bool,
}

/// A simple container to center your content horizontally.
///
/// [https://bulma.io/documentation/layout/container/](https://bulma.io/documentation/layout/container/)
pub struct Container {
    props: ContainerProps,
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _ctx: Context) -> bool {
        false
    }

    fn changed(&mut self, ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("container");
        classes.push(&self.props.classes);
        if self.props.fluid {
            classes.push("is-fluid");
        }
        html! {
            <div class={classes}>
                {self.props.children.clone()}
            </div>
        }
    }
}
