use super::{toaster::Toaster, ToastPosition, ToastIntent, ToasterInjection};
use leptos::{context::Provider, prelude::*};

#[component]
pub fn ToasterProvider(
    /// The position the toast should render.
    #[prop(optional)]
    position: ToastPosition,
    /// The intentthe toast should render.
    #[prop(optional)]
    intent: ToastIntent,
    children: Children,
) -> impl IntoView {
    let (injection, receiver) = ToasterInjection::channel();
    view! {
        <Toaster receiver position intent />
        <Provider value=injection>{children()}</Provider>
    }
}
