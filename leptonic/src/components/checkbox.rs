use leptos::*;

use crate::{components::icon::Icon, OptMaybeSignal, Out};

#[component]
pub fn Checkbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] set_checked: Out<bool>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(default = icondata::BsCheck2)] checked_icon: icondata::Icon,
    /// Arbitrary additional attributes.
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let disabled = move || disabled.0.as_ref().map_or(false, SignalGet::get);

    view! {
        <leptonic-checkbox
            {..attributes}
            id=id
            class=class
            style=style
            role="checkbox"
            aria-checked=move || match checked.get() { true => "true", false => "false" }
            aria-disabled=move || match disabled() { true => "true", false => "false" }
            tabindex="0"
            on:click=move |_e| {
                if !disabled() {
                    set_checked.set(!checked.get_untracked())
                }
            }
        >
            <Icon icon=checked_icon style=move || match checked.get() {
                true => "display: inherit",
                false => "display: none",
            } />
        </leptonic-checkbox>
    }
}
