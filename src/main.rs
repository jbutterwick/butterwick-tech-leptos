use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "+"
        </button>
        <p>
            <strong
                class:red=move || count() % 2 == 1
            >
                "Current count: "
                {count}
            </strong>
        </p>
        <button
            on:click=move |_| {
                set_count.update(|n| *n -= 1);
            }
        >
            "-"
        </button>

        // NOTE: self-closing tags like <br> need an explicit /
        <br/>

        // We'll update this progress bar every time `count` changes
        <progress
            // static attributes work as in HTML
            max="50"

            // passing a function to an attribute
            // reactively sets that attribute
            // signals are functions, so this <=> `move || count.get()`
            value=count
        >
        </progress>
        <br/>

        // This progress bar will use `double_count`
        // so it should move twice as fast!
        <progress
            max="50"
            // derived signals are functions, so they can also
            // reactive update the DOM
            value=double_count
        >
        </progress>
        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}