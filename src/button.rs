use yew::prelude::*;

pub struct Button {
    action: Action,
}


impl Button {
    fn content(&self) -> Html {
        match self.action {
            Action::Add(n) => html! {format!("{}", n)},
            Action::Reset => html!{
                <sl-icon name="x-lg" />
            },
            Action::Delete => html!{
                <sl-icon name="chevron-left" />
            },
        }
    }

    fn variant(&self) -> AttrValue {
        match self.action {
            Action::Add(_) => AttrValue::from("default"),
            Action::Reset => AttrValue::from("danger"),
            Action::Delete => AttrValue::from("warning"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub action: Action,
    pub onclick: Callback<Action>,
}

#[derive(PartialEq, Clone)]
pub enum Action {
    Add(u8),
    Reset,
    Delete,
}


impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            action: ctx.props().action.clone(),
        }
    }



    fn view(&self, ctx: &Context<Self>) -> Html {

        let action = self.action.clone();
        let variant = self.variant();
        let onclick = ctx.props().onclick.reform(move |_| action.clone());
        let outline = match self.action {
            Action::Reset | Action::Delete => Some("outline"),
            _ => None
        };

        html! {
            <sl-button {onclick} {variant} {outline}>{self.content()}</sl-button>
        }
    }
}