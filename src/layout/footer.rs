use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A simple responsive footer which can include anything.
///
/// [https://bulma.io/documentation/layout/footer/](https://bulma.io/documentation/layout/footer/)
pub struct Footer {
    props: FooterProps,
}

impl Component for Footer {
    type Message = ();
    type Properties = FooterProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _ctx: Context,_: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("footer");
        classes.push(&self.props.classes);
        html! {
            <footer class={classes}>
                {self.props.children.clone()}
            </footer>
        }
    }
}
