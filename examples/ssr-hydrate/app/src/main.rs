use sycamore::prelude::*;

pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    // When `hydrate` feature flag is absent, we assume to be running through `trunk serve` and
    // render app with two counter components to showcase how it looks.
    #[cfg(not(feature="hydrate"))]
    {
        log::debug!("render application component");
        sycamore::render(|cx| view! { cx, app::App {
            app::Counter(label=String::from("rendered counter 1"), value=0)
            app::Counter(label=String::from("rendered counter 2"), value=0)
        }});
    }

    // When `hydrate` feature flag is present, we assume to be running through backend service and
    // hydrate the rendered HTML. This is the main mode of operation for this example.
    //
    // Note that we are *not* passing any `Counter` components to `App`. Since the HTML is
    // generated by backend, the `App` should just hydrates any `Counter`s that were rendered.
    #[cfg(feature="hydrate")]
    {
        log::debug!("hydrate application component");
        sycamore::hydrate(|cx| view! { cx, app::App {
            // FIXME: Workaround for https://github.com/sycamore-rs/sycamore/issues/588
            ""

            // Passing `Counter` component to `App` here would add it to the list received from
            // backend. Or it would be ignored. All depending on the hydration logic in `App`
            // component, but definitely nothing we would like
            //
            // app::Counter(label=String::from("hydrated counter 1"), value=1)
        }});
    }
}
