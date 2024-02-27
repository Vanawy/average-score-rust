use std::ops::Range;
use yew::prelude::*;
use crate::button::{Button, ButtonAction};

pub struct Calculator {
    sum: u64,
    count: usize,
    average: f64,
    scores: Vec<u64>,
}

pub enum CalculatorMsg {
    ButtonClicked(ButtonAction),
}

impl Calculator {
    fn recalculate(&mut self) {
        self.count = self.scores.len();
        self.sum = self.scores.iter().sum();
        self.average = (self.sum) as f64 / (self.count) as f64;
    }
}

impl Component for Calculator {
    type Message = CalculatorMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            sum: 0,
            count: 0,
            average: 0.0,
            scores: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CalculatorMsg::ButtonClicked(action) => {
                match action {
                    ButtonAction::Add(n) => {
                        self.scores.push(n as u64);
                    },
                    ButtonAction::Reset => {
                        self.scores = Vec::new();
                    },
                    ButtonAction::Delete => {
                        self.scores.pop();
                    },
                };
            }
        }
        self.recalculate();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut scores = String::from("-");
        let mut summary = String::from("0 / 0");
        let mut avg = String::from("-");

        if self.count > 0 {
            scores = self.scores.clone().into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            summary = format!("{} / {}", self.sum, self.count);
            avg = format!("{:.3}", self.average);
        }


        let mut actions = Vec::new();

        Range::from(1..10).for_each(|i| actions.push(ButtonAction::Add(i)));
        actions.push(ButtonAction::Reset);
        actions.push(ButtonAction::Add(10));
        actions.push(ButtonAction::Delete);

        let buttons = actions.iter().map(|action| {
            html! {
                <Button
                action={action.clone()}
                onclick={ctx.link().callback(move |action| CalculatorMsg::ButtonClicked(action))}
                />
            }
        }).collect::<Html>();

        html! {
            <sl-card id="main">
                <p id="scores">{scores}</p>
                <sl-divider></sl-divider>
                <p id="description">{summary}</p>
                <p id="average" style="font-size: var(--sl-font-size-2x-large)">{avg}</p>

                <div id="buttons" style="text-align: center;">
                    { buttons }
                </div>
            </sl-card>
        }
    }
}