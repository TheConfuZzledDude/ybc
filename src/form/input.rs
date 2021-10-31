#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use web_sys::HtmlInputElement;
use yew::events::Event;
use yew::html::Scope;
use yew::prelude::*;
use yew::TargetCast;
use yewtil::NeqAssign;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The input type of this component.
    #[prop_or_else(|| InputType::Text)]
    pub r#type: InputType,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: bool,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Input {
    props: InputProps,
    link: Scope,
}

impl Component for Input {
    type Message = String;
    type Properties = InputProps;

    fn create(ctx: Context) -> Self {
        Self { props: ctx.props(), link: ctx.link() }
    }

    fn update(&mut self, _ctx: Context, msg: Self::Message) -> bool {
        self.props.update.emit(msg);
        false
    }

    fn changed(&mut self, ctx: Context) -> bool {
        self.props.neq_assign(ctx.props())
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("input");
        classes.push(&self.props.classes);
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if self.props.rounded {
            classes.push("is-rounded");
        }
        if self.props.loading {
            classes.push("is-loading");
        }
        if self.props.r#static {
            classes.push("is-static");
        }
        html! {
            <input
                name={self.props.name.clone()}
                value={self.props.value.clone()}
                oninput={self.link.callback(|e: Event| e.target_unchecked_into::<HtmlInputElement>().value())}
                class={classes}
                type={self.props.r#type.to_string()}
                placeholder={self.props.placeholder.clone()}
                disabled={self.props.disabled}
                readonly={self.props.readonly}
                />
        }
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq)]
pub enum InputType {
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "password")]
    Password,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "tel")]
    Tel,
}
