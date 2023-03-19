use sycamore::prelude::*;

#[derive(Prop)]
pub struct AppProps<'cx, G: Html> {
    pub children: Children<'cx, G>,
}

#[component]
pub fn App<'cx, G: Html>(cx: Scope<'cx>, props: AppProps<'cx, G>) -> View<G> {

    #[cfg(feature="hydrate")]
    {
        use std::any::TypeId;
        if TypeId::of::<G>() == TypeId::of::<HydrateNode>() {
            log::debug!("hydrate app");
            while let Some(el) =  sycamore::web::hydrate::get_next_element() {
                log::debug!("found el");
                if let Some(pair) =  sycamore::utils::hydrate::get_current_id() {
                    log::debug!("{:?}", pair);
                }
            }
            view!{cx, "foo"}
        }
        else {
            let children = props.children.call(cx);
            view! { cx, div {
                (children)
            }}
        }
    }

    #[cfg(not(feature="hydrate"))]
    {
        let children = props.children.call(cx);
        view! { cx, div {
            (children)
        }}
    }
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

    log::debug!("hydrate counter");

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
