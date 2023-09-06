#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet, MaterialIconVariant};

struct IsLoggedIn(bool);
struct IsMenuOpened(bool);

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    // TODO: Investigate errors when changing IsLoggedIn state.
    use_shared_state_provider(cx, || IsLoggedIn(false));
    use_shared_state_provider(cx, || IsMenuOpened(true));

    let is_logged_in_context = use_shared_state::<IsLoggedIn>(cx).unwrap();

    cx.render(rsx! (

        MaterialIconStylesheet { variant: MaterialIconVariant::Outlined }

        if is_logged_in_context.read().0 {
            Inbox(cx)
        } else {
            LoginScreen(cx)
        }
    ))
}

fn LoginScreen(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "login_screen", LoginContainer(cx) }
    })
}

fn LoginContainer(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "login_container",
            MaterialIcon { name: "email", size: 80, color: "#000000" }
            h1 {
                padding: "0",
                // margin: "10px 0 0 15px",
                text_align: "right",
                max_width: "360px",
                text_align: "center",
                "Create a Freemail account"
            }
            LoginForm(cx)
        }
    })
}

fn LoginForm(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "form_group",
            span { "https://" }
            input { class: "form_field", "type": "text", placeholder: "domain.tld" }
        }
        div { class: "form_group",
            input { class: "form_field" }
            span { "@freemail.com" }
        }
        Divider(cx),
        div { class: "form_group",
            span { "Import key file" }
            input {
                class: "form_field",
                "type": "file",
                name: "keypair-file",
                placeholder: "************"
            }
        }
        p {
            padding: "0",
            margin: "0 0 30px 0",
            text_align: "right",
            width: "100%",
            max_width: "360px",
            text_align: "left",
            color: "#AFB9CF",
            font_size: "12px",
            "If a key file is not imported, one will be generated for you."
        }
        // h6{
        //     padding: "0",
        //     margin: "0 0 15px 0",
        //     text_align: "right",
        //     "OR",
        // }
        Button(cx)
    })
}

fn Divider(cx: Scope) -> Element {
    cx.render(rsx! {
        hr {
            class: "hr-dashed",
            width: "100%",
            margin: "10px 0 25px 0",
            // border: "none",
            // border_bottom: "dashed 1px #ebebeb",

            border: "none",
            height: "1.2px",
            background: "#5D6576",
            background: "repeating-linear-gradient(90deg,#AFB9CF,#AFB9CF 6px,transparent 6px,transparent 12px)"
        }
    })
}

fn Inbox(cx: Scope) -> Element {
    // let inbox_style = r#"
    // display: grid;
    // grid: 60px 1fr / 60px calc(100% - 60px);
    // transition: 0.3s ease;
    // user-select: none;
    // background: #fff;
    // "#;
    let is_menu_opened_context = use_shared_state::<IsMenuOpened>(cx).unwrap();

    let is_opened = if is_menu_opened_context.read().0 {
        "inbox__submenu-list__opened"
    } else {
        ""
    };

    let is_content_opened = if is_menu_opened_context.read().0 {
        "inbox__content_opened"
    } else {
        ""
    };

    cx.render(rsx! {
        div {
            // style:"{inbox_style}",
            class: "inbox",
            div { class: "inbox__menu-button",
                MaterialIcon { name: "mail", size: 24, color: "#8da2b5" }
            }

            div { class: "inbox__menu-list" }
            div { class: "inbox__submenu-list {is_opened}" }
            div { class: "inbox__content {is_content_opened}",
                div { class: "emails",
                    div {
                        display: "flex",
                        align_items: "center",
                        border_bottom: "1.2px solid #e0e5eb",
                        padding: "16px",
                        HamburgerButton(cx),
                        span { padding: "20px", "Uncategorised" }
                    }
                    div {
                        display: "flex",
                        justify_content: "space-between",
                        align_items: "center",
                        border_bottom: "1.2px solid #e0e5eb",
                        padding: "8px 16px 8px 16px",
                        font_size: "11px",
                        color: "#62778c",
                        div { display: "flex", align_items: "center",
                            MaterialIcon { name: "inbox", size: 16, color: "#62778c" }
                            span { padding_left: "5px", "Open (20) ▾" }
                        }
                        div { span { "Newest ▴" } }
                    }

                    div { display: "block", overflow: "scroll", cursor: "pointer",
                        (0..60).map(|i| rsx!{div {
                            class: "email_preview",
                            display: "flex",
                            align_items: "center",
                            padding: "16px",
                            border_bottom: "0.7px solid #e0e5eb",
                            "Email {i}",
                        }})
                    }
                }
                div {
                    class: "emails",
                    border_left: "1.2px solid #e0e5eb",
                    border_right: "1.2px solid #e0e5eb",
                    div {
                        display: "flex",
                        align_items: "center",
                        border_bottom: "1.2px solid #e0e5eb",
                        padding: "16px",
                        span { padding: "20px", "john.doe@example.com" }
                    }
                }
                div { class: "emails", div { border_bottom: "1.2px solid #e0e5eb" } }
            }
        }
    })
}

fn Button(cx: Scope) -> Element {
    let is_logged_in_context = use_shared_state::<IsLoggedIn>(cx).unwrap();

    cx.render(rsx! (
        button {
            class: "btn btn-primary",
            width: "100%",
            border_radius: "6px",
            onclick: move |_| {
                is_logged_in_context.write().0 = true;
            },
            "Create"
        }
    ))
}

fn HamburgerButton(cx: Scope) -> Element {
    let is_menu_opened_context = use_shared_state::<IsMenuOpened>(cx).unwrap();

    let inbox_style = r#"
    // width:60px;
    cursor: pointer;
    "#;

    cx.render(rsx! (
        a {
            style: "{inbox_style}",
            border_radius: "6px",
            onclick: move |_| {
                let is_enabled = is_menu_opened_context.write().0 == true;
                is_menu_opened_context.write().0 = !is_enabled;
            },

            MaterialIcon {
                name: if is_menu_opened_context.read().0 { "close" } else { "menu" },
                size: 24,
                color: "#62778c"
            }
        }
    ))
}
