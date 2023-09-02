use leptos::*;

#[component]
pub fn Banner(cx: Scope) 
  -> impl IntoView {
    view! { cx,
        <h2>"Weblum Photos"</h2>
    }.into_view(cx)
}