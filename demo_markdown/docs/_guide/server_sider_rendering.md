# Server Sider Rendering

## SSRMountStyleProvider

In SSR mode, please use `SSRMountStyleProvider` component to wrap `html` tag.

Refer to [ssr_axum/src/app.rs](https://github.com/thaw-ui/thaw/blob/main/examples/ssr_axum/src/app.rs).

```rust
view! {
    <SSRMountStyleProvider>
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=leptos_options.clone() />
                <HydrationScripts options=leptos_options.clone()/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    </SSRMountStyleProvider>
}
```

For [start-axum](https://github.com/leptos-rs/start-axum), it is in the [src/app.rs](https://github.com/leptos-rs/start-axum/blob/main/src/app.rs#L9) file.

For [start-actix](https://github.com/leptos-rs/start-actix), it is in the [src/main.rs](https://github.com/leptos-rs/start-actix/blob/943b1ad428072267f32217de4c62896f2bf70459/src/main.rs#L33) file.

## cargo-leptos

If you use [cargo-leptos](https://github.com/leptos-rs/cargo-leptos), Remember to add thaw to your `Cargo.toml` file in the corresponding feature, e.g.

```toml
[features]
...
hydrate = [..., "thaw/hydrate"]
ssr = [
  ...,
  "thaw/ssr",
]
```
