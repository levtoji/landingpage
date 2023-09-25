use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PillProps {
    pub color: &'static str,
    pub hover_color: &'static str,
    pub text: &'static str,
}

#[function_component]
pub fn Pill(props: &PillProps) -> Html {
    let classes = classes! {vec![{props.color}, {props.hover_color}], "text-white", "font-bold", "py-2", "px-2", "rounded-full"};
    html! {
    <div class={classes}>{props.text}</div>
    }
}
