use sycamore::prelude::*;

#[derive(Props)]
pub struct AccessibleInputProps<'cx, G: Html> {

    // component attribute, will be handled localy and *not* passed-thorugh
    label_id: &'static str,

    // collection of passthrough attributes
    attributes: Attributes<'cx, G>,

    // collection of passthrough children nodes/components
    children: Children<'cx, G>,
}

#[component]
fn AccessibleSearchBox<'cx, G: Html>(
    cx: Scope<'cx>,
    props: AccessibleInputProps<'cx, G>,
) -> View<G> {

    // we remove attributes that are handled by component and should not be passed through
    props
        .attributes
        .exclude_keys(&["aria-role", "aria-labelledby"]);

    // render all child nodes into temporary variable
    let children = props.children.call(cx);

    // render component as `input` node using remaining attributes and wrapping rendered children
    // nodes.
    view! { cx,
        input(aria-role = "searchbox", aria-labelledby = props.label_id, ..props.attributes) {
            (children)
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            "Passthrough attributes demo"

            // attributes prefixed with attr: will be accumulated into `attributes` field
            label(id = "searchbox1_label") { "Search Box 1" }
            AccessibleSearchBox(label_id = "searchbox1_label", attr:style="background-color:slategray;") {}
            label(id = "searchbox2_label") { "Search Box 2" }
            AccessibleSearchBox(label_id = "searchbox2_label", attr:style="background-color:gray;") { }
        }
    }
}

fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
