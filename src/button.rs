use yew::prelude::*;

pub struct Button {
    action: ButtonAction
}


impl Button {
    fn content(&self) -> Html {
        match self.action {
            ButtonAction::Add(n) => html! {format!("{}", n)},
            ButtonAction::Reset => html!{
                <sl-icon name="x-lg" />
            },
            ButtonAction::Delete => html!{
                <sl-icon name="chevron-left" />
            },
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub action: ButtonAction,
    pub onclick: Callback<ButtonAction>
}

#[derive(PartialEq, Clone)]
pub enum ButtonAction {
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
        let onclick = ctx.props().onclick.reform(move |_| action.clone());

        html! {
            <sl-button {onclick}>{self.content()}</sl-button>
        }
    }
}