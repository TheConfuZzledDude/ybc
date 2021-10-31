use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Bulma’s most basic spacer block
///
/// [https://bulma.io/documentation/elements/block/](https://bulma.io/documentation/elements/block/)
pub struct Block {
    props: BlockProps,
}

impl Component for Block {
    type Message = ();
    type Properties = BlockProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _ctx:Context, _: Self::Message) -> bool {
        false
    }

    fn changed(&mut self,ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("block");
        classes.push(&self.props.classes);
        html! {
            <div class={classes}>
                {self.props.children.clone()}
            </div>
        }
    }
}
