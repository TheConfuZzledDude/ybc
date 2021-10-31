#![allow(clippy::redundant_closure_call)]

use yew::{html::IntoPropValue, prelude::*};
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "nav".into())]
    pub tag: String,
}

/// A multi-purpose horizontal level, which can contain almost any other element.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct Level {
    props: LevelProps,
}

impl Component for Level {
    type Message = ();
    type Properties = LevelProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _ctx: Context, _: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("level");
        classes.push(&self.props.classes);
        html! {
            <@{self.props.tag.clone()} class={classes}>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelLeftProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A container for level elements to be grouped to the left of the container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelLeft {
    props: LevelLeftProps,
}

impl Component for LevelLeft {
    type Message = ();
    type Properties = LevelLeftProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _ctx: Context, _: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("level-left");
        classes.push(&self.props.classes);
        html! {
            <@{self.props.tag.clone()} class={classes}>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelRightProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A container for level elements to be grouped to the right of the container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelRight {
    props: LevelRightProps,
}

impl Component for LevelRight {
    type Message = ();
    type Properties = LevelRightProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _ctx: Context, _: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("level-right");
        classes.push(&self.props.classes);
        html! {
            <@{self.props.tag.clone()} class={classes}>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// An individual element of a level container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelItem {
    props: LevelItemProps,
}

impl Component for LevelItem {
    type Message = ();
    type Properties = LevelItemProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self,_ctx: Context, _: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx:Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("level-item");
        classes.push(&self.props.classes);
        html! {
            <@{self.props.tag.clone()} class={classes}>
                {self.props.children.clone()}
            </@>
        }
    }
}
