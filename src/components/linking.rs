use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkingProps {
    pub text: &'static str,
    pub link: &'static str,
}
#[function_component]
pub fn Linking(props: &LinkingProps) -> Html {
    html! {
        <a href={props.link} class="text-white bg-gray-700 hover:bg-gray-800 focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-gray-600 dark:hover:bg-gray-700 focus:outline-none dark:focus:ring-gray-800">{props.text}</a>
    }
}
