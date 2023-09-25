use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkingProps {
    pub text: &'static str,
    pub link: &'static str,
    pub children: Children,
}
#[function_component]
pub fn Linking(props: &LinkingProps) -> Html {
    html! {
        <>
        <a href={props.link} alt={props.text} class="flex text-white bg-lime-500 hover:bg-lime-700 font-semibold py-2 px-4 border border-2 border-lime-500 hover:border-transparent rounded">{ for props.children.iter() }</a>
        </>
    }
}
