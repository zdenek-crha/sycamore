use sycamore::prelude::*;

#[derive(Prop)]
pub struct AppProps<'cx, G: Html> {
    pub children: Children<'cx, G>,
}

#[component]
pub fn App<'cx, G: Html>(cx: Scope<'cx>, props: AppProps<'cx, G>) -> View<G> {
    let children = props.children.call(cx);

    view! { cx, div {
        (children)
    }}
}


#[derive(Prop)]
pub struct CounterProps {
    pub label: String,
    pub value: i32,
}

#[component]
pub fn Counter<G: Html>(cx: Scope, props: CounterProps) -> View<G> {
    let state = create_signal(cx, props.value);
    let increment = |_| state.set(*state.get() + 1);
    let decrement = |_| state.set(*state.get() - 1);

    view! { cx,
        div {
            span { (props.label) }
            " "
            button(on:click=decrement) { "-" }
            input(value=state.get())
            button(on:click=increment) { "+" }
        }
    }
}
