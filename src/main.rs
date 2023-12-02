#![allow(non_snake_case)]

use chrono::{DateTime, Duration, Utc};
use dioxus::{html::h1, prelude::*};
use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet, MaterialIconVariant};

struct IsLoggedIn(bool);
struct IsMenuOpened(bool);

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("sup");

    dioxus_web::launch(App);
}

#[derive(Clone, Copy)]
enum ThemeMode {
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy)]
enum IsOpened {
    EmailComposer,
    Email,
    None,
}

fn App(cx: Scope) -> Element {
    let create_eval = use_eval(cx);
    use_shared_state_provider(cx, || IsLoggedIn(true));
    use_shared_state_provider(cx, || IsMenuOpened(true));
    use_shared_state_provider(cx, || IsOpened::None);

    let is_logged_in_context = use_shared_state::<IsLoggedIn>(cx).unwrap();

    let theme_mode = ThemeMode::System;

    let eval_provider = use_eval(cx);

    let _ = match theme_mode {
        ThemeMode::Light => eval_provider(
            r#"
            const root = document.querySelector(':root');
            root.classList.remove('dark');
            root.classList.add('light');
        "#,
        ),
        ThemeMode::Dark => eval_provider(
            r#"
            const root = document.querySelector(':root');
            root.classList.remove('light');
            root.classList.add('dark');
        "#,
        ),
        ThemeMode::System => eval_provider(
            r#"
            const root = document.querySelector(':root');
            root.classList.remove('dark');
            root.classList.remove('light');
        "#,
        ),
        // _ => todo!(),
        // None => todo!(),
    };

    // let future = use_future(cx, (), |_| {
    //     to_owned![eval_provider];

    //     async move {
    //         let eval = eval_provider(
    //             r#"
    //             const root = document.querySelector(':root');
    //             root.classList.add('dark');
    //         "#,
    //         )
    //         .unwrap();

    //         eval.send("Hi from Rust!".into()).unwrap();
    //         let res = eval.recv().await.unwrap();
    //         println!("{:?}", eval.await);
    //         res
    //     }
    // });

    // match future.value() {
    //     Some(v) => cx.render(rsx!( p { "{v}" } )),
    //     _ => cx.render(rsx!( p { "hello" } )),
    //     None => todo!(),
    // }

    cx.render(rsx! {
        MaterialIconStylesheet { variant: MaterialIconVariant::Outlined }
        if is_logged_in_context.read().0 {
            rsx!{Inbox{}}
        } else {
            rsx!{LoginScreen{}}
        }
    })
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
        Button { title: String::from("Create") }
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
    let middle_view = use_shared_state::<IsOpened>(cx).unwrap();

    let is_middle_view_opened = match *middle_view.read() {
        IsOpened::Email => "is_middle_view_opened",
        IsOpened::EmailComposer => "is_middle_view_opened",
        IsOpened::None => "",
    };

    let is_opened = if is_menu_opened_context.read().0 {
        "menu_wrapper__opened"
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

            header { class: "menu_wrapper {is_opened}",
                div { class: "inbox__menu-button", MaterialIcon { name: "mail", size: 24, color: "#8da2b5" } }
                div { class: "inbox__menu-list",
                    div {
                        onclick: move |_| {
                            *middle_view.write() = IsOpened::EmailComposer;
                        },
                        MaterialIcon { name: "add", size: 24, color: "#8da2b5" }
                    }
                    div {
                        grid_row: "-2",
                        onclick: move |_| {
                            *middle_view.write() = IsOpened::EmailComposer;
                        },
                        MaterialIcon { name: "settings", size: 24, color: "#8da2b5" }
                    }
                }
                div { class: "inbox__submenu-list ",
                    ul {
                        li {
                            div { "Inbox" }
                            div { "92" }
                        }
                        li { "Drafts" }
                        li { "Sent" }
                        li {
                            div { "Starred" }
                            div { "33" }
                        }
                        li { "Archive" }
                        li { "Spam" }
                        li { "Trash" }
                        li {
                            div { "All mail" }
                            div { "92" }
                        }
                        li { "Folders" }
                        li { "Labels" }
                    }
                }
            }

            main { class: "inbox__content {is_content_opened} {is_middle_view_opened}",
                div { class: "emails",
                    div { class: "border_bottom", display: "flex", align_items: "center", padding: "16px",
                        HamburgerButton(cx),
                        span { padding_left: "20px", "Uncategorised" }
                    }
                    div {
                        class: "border_bottom",
                        display: "flex",
                        justify_content: "space-between",
                        align_items: "center",
                        padding: "8px 16px 8px 16px",
                        font_size: "11px",
                        color: "#62778c",
                        div { display: "flex", align_items: "center",
                            // MaterialIcon { name: "inbox", size: 16, color: "#62778c" }
                            MaterialIcon { name: "filter_alt", size: 16, color: "#62778c" }
                            span { padding_left: "5px", "Open (20) ▾" }
                        }
                        div { span { "Newest ▴" } }
                    }

                    div { display: "block", overflow: "scroll", cursor: "pointer",
                        (0..60).map(|i| rsx!{
                        EmailTile{
                            sender: "Jane Doe".to_string(),
                            title: String::from(format!("New meeting with the corporate tomorrow")),
                            paragraph: "Paragraph".to_string(),
                            timestamp: Utc::now() - Duration::minutes(i),
                        }
                        })
                    }
                }
                match *middle_view.read() {
                    IsOpened::EmailComposer => rsx!{EmailComposer { is_online: true }},
                    IsOpened::Email => rsx!{EmailReader { is_online: true }},
                    IsOpened::None => rsx!{""},
                    // _ => rsx!{""},
                }
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct EmailProps {
    sender: String,
    title: String,
    paragraph: String,
    timestamp: DateTime<Utc>,
}

fn EmailTile(cx: Scope<EmailProps>) -> Element {
    let end_time = Utc::now().time();
    let diff = end_time - cx.props.timestamp.time();
    let middle_view = use_shared_state::<IsOpened>(cx).unwrap();

    cx.render(rsx!(
        div {
            class: "email_tile",
            onclick: move |_| {
                *middle_view.write() = IsOpened::Email;
            },
            Avatar { is_online: true }
            div { width: "100%", padding_left: "16px",
                div { class: "sender",
                    div { b { "{cx.props.sender}" } }
                    div { color: "#62778c", font_size: "13px", format!("{}m",diff.num_minutes()) }
                }
                div {
                    // class: "title",
                    "{cx.props.title}"
                }
            }
        }
    ))
}

#[derive(Props, PartialEq)]
struct AvatarProps {
    is_online: bool,
}

fn Avatar(cx: Scope<AvatarProps>) -> Element {
    cx.render(rsx!(
        div { class: "avatar",
            img {
                src: "https://images.unsplash.com/photo-1542103749-8ef59b94f47e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1350&q=80",
                alt: "Avatar image"
            }
            span { class: "online_status" }
        }
    ))
}

#[derive(Props, PartialEq)]
struct EmailComposerProps {
    is_online: bool,
}

fn EmailComposer(cx: Scope<EmailComposerProps>) -> Element {
    cx.render(rsx!(
        div { class: "composer",
            div { class: "border_bottom", display: "flex", align_items: "center", padding: "16px",
                HamburgerButton {}
                div { width: "16px" }
                CloseBackButton {}
                span { padding: "20px", "New Message" }
            }
            div { class: "border_bottom", padding: "10px 20px 30px 20px",
                div {
                    width: "100%",
                    display: "flex",
                    "flex-direction": "row",
                    "flex-wrap": "nowrap",
                    padding: "0 16px 0 16px",
                    label { "for": "from", "From" }
                    div {
                        class: "border_bottom",
                        // min_height: "3em",
                        padding: "10px 16px 10px 16px",
                        width: "100%",
                        input {
                            "type": "email",
                            name: "email",
                            id: "email",
                            placeholder: "john.doe@example.com"
                        }
                    }
                }

                div {

                    width: "100%",

                    display: "flex",

                    "flex-direction": "row",

                    "flex-wrap": "nowrap",

                    padding: "0 16px 0 16px",
                    label { "for": "email", "To" }
                    div {
                        class: "border_bottom",
                        display: "flex",

                        "flex-direction": "row",

                        "flex-wrap": "nowrap",
                        // min_height: "3em",
                        padding: "10px 6px 10px 16px",
                        width: "100%",
                        input {
                            "type": "email",
                            name: "email",
                            id: "email",
                            placeholder: "Email address"
                        }
                        span { padding: "0 5px", "CC" }
                        span { padding: "0 5px", "BCC" }
                        a { cursor: "pointer", height: "18px", border_radius: "6px", padding: "0 10px",
                            MaterialIcon { name: "person_add", size: 18, color: "#62778c" }
                        }
                    }
                }

                div {

                    width: "100%",

                    display: "flex",

                    "flex-direction": "row",

                    "flex-wrap": "nowrap",

                    padding: "0 16px 0 16px",
                    label { "for": "subject", "Subject" }
                    div {
                        class: "border_bottom",
                        // min_height: "3em",
                        padding: "10px 16px 10px 16px",
                        width: "100%",
                        input {
                            "type": "subject",
                            name: "subject",
                            id: "subject",
                            placeholder: "Subject"
                        }
                    }
                }
            }
            textarea { placeholder: "A quick brown fox jumped over the lazy dog..." }
            div {
                class: "border_top",
                width: "100%",
                display: "flex",
                "flex-direction": "row",
                "flex-wrap": "nowrap",
                align_items: "center",
                padding: "16px 30px 24px 30px",
                a { cursor: "pointer", height: "24px", border_radius: "6px", padding: "0 10px",
                    MaterialIcon { name: "delete", size: 24, color: "#62778c" }
                }
                a { cursor: "pointer", height: "24px", border_radius: "6px", padding: "0 10px",
                    MaterialIcon { name: "enhanced_encryption", size: 24, color: "#62778c" }
                }
                div { width: "200%" }
                a { cursor: "pointer", height: "24px", border_radius: "6px", padding: "0 10px",
                    MaterialIcon { name: "attach_file", size: 24, color: "#62778c" }
                }
                Button { title: String::from("Send") }
            }
        }
    ))
}

#[derive(Props, PartialEq)]
struct EmailReaderProps {
    is_online: bool,
}

fn EmailReader(cx: Scope<EmailReaderProps>) -> Element {
    cx.render(rsx!(
        div { class: "email",
            div {
                class: "border_bottom",
                display: "flex",
                align_items: "center",
                padding: "16px 16px 16px 30px",
                CloseBackButton {}
                div { style: r#"zoom: 0.7; padding-top: 5px; padding-left:56px;"#, Avatar { is_online: true } }
                span { style: r#"overflow:hidden;padding-left:20px; text-overflow: ellipsis;flex-wrap: nowrap; text-wrap: nowrap;"#,
                    "jane.doe@example.com"
                }
                div {
                    width: "100%",
                    display: "flex",
                    "flex-direction": "row",
                    "flex-wrap": "nowrap",
                    align_items: "center",
                    justify_content: "space-between",
                    a { cursor: "pointer", height: "20px", border_radius: "6px", padding: "0 10px",
                        MaterialIcon { name: "shield", size: 20, color: "#62778c" }
                    }
                    div {
                        a { cursor: "pointer", height: "24px", border_radius: "6px", padding: "0 10px",
                            MaterialIcon { name: "star", size: 24, color: "#62778c" }
                        }
                        a { cursor: "pointer", height: "24px", border_radius: "6px", padding: "0 10px",
                            MaterialIcon { name: "delete", size: 24, color: "#62778c" }
                        }
                    }
                }
            }
            div { class: "border_bottom", padding: "10px 10px 10px 20px",
                // div {
                //     width: "100%",
                //     display: "flex",
                //     "flex-direction": "row",
                //     "flex-wrap": "nowrap",
                //     padding: "0 16px 0 16px",
                //     label { "for": "from", "From" }
                //     div { padding: "10px 16px 10px 16px", width: "100%", "jane.doe@example.com" }
                // }

                div {
                    width: "100%",
                    display: "flex",
                    "flex-direction": "row",
                    "flex-wrap": "nowrap",
                    padding: "0 16px 0 16px",
                    label { "for": "email", "To" }
                    div {
                        display: "flex",

                        "flex-direction": "row",

                        "flex-wrap": "nowrap",
                        // min_height: "3em",
                        padding: "10px 6px 10px 16px",
                        width: "100%",
                        "john.doe@example.com, john.doe@example.com"
                    }
                }
            }

            div { padding: "40px",
                h2 { margin_top: "0", "New meeting with corporate tomorrow" }
                pre { padding: "0", margin: "0", white_space: "pre-wrap", word_wrap: "break-word",
                    "Lorem ipsum dolor, 
                    
                    \nSit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                    \nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                    
                    \nSed ut perspiciatis!"
                }
            }
            div {
                class: "border_top",
                width: "100%",
                display: "flex",
                "flex-direction": "row",
                "flex-wrap": "nowrap",
                align_items: "center",
                padding: "16px 30px 24px 30px",
                a { cursor: "pointer", height: "24px", border_radius: "6px", padding: "0 10px",
                    MaterialIcon { name: "delete", size: 24, color: "#62778c" }
                }
                a { cursor: "pointer", height: "24px", border_radius: "6px", padding: "0 10px",
                    MaterialIcon { name: "enhanced_encryption", size: 24, color: "#62778c" }
                }
                div { width: "200%" }
                a { cursor: "pointer", height: "24px", border_radius: "6px", padding: "0 10px",
                    MaterialIcon { name: "attach_file", size: 24, color: "#62778c" }
                }
                Button { title: String::from("Reply") }
            }
        }
    ))
}

fn EmailDetails(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "email_details", div { class: "border_bottom" } }
    ))
}

#[derive(Props, PartialEq)]
struct ButtonProps {
    title: String,
}

fn Button(cx: Scope<ButtonProps>) -> Element {
    let is_logged_in_context = use_shared_state::<IsLoggedIn>(cx).unwrap();

    cx.render(rsx! (
        button {
            class: "btn btn-primary",
            width: "100%",
            border_radius: "6px",
            onclick: move |_| {
                is_logged_in_context.write().0 = true;
            },
            "{cx.props.title}"
        }
    ))
}

#[derive(Props, PartialEq)]
struct FloatingButtonProps {
    title: String,
}

fn FloatingButton(cx: Scope<FloatingButtonProps>) -> Element {
    let is_logged_in_context = use_shared_state::<IsLoggedIn>(cx).unwrap();

    cx.render(rsx! (
        button {
            class: "btn btn-primary btn-floating",
            width: "100%",
            border_radius: "6px",
            onclick: move |_| {
                is_logged_in_context.write().0 = true;
            },
            "{cx.props.title}"
        }
    ))
}

fn HamburgerButton(cx: Scope) -> Element {
    let is_menu_opened_context = use_shared_state::<IsMenuOpened>(cx).unwrap();

    let inbox_style = r#"
    // width:60px;
    cursor: pointer;
    height: 24px;
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

fn CloseBackButton(cx: Scope) -> Element {
    let middle_view = use_shared_state::<IsOpened>(cx).unwrap();

    let inbox_style = r#"
    // width:60px;
    cursor: pointer;
    height: 24px;
    "#;

    cx.render(rsx! (
        a {
            style: "{inbox_style}",
            border_radius: "6px",
            onclick: move |_| {
                *middle_view.write() = IsOpened::None;
            },

            MaterialIcon { name: "arrow_back", size: 24, color: "#62778c" }
        }
    ))
}

#[derive(Props, PartialEq)]
struct GreetPersonProps {
    person: String,
}

#[allow(non_snake_case)]
fn GreetPerson<'a>(cx: Scope<'a, GreetPersonProps>) -> Element {
    __greet_person(cx)
}

#[inline(always)]
fn __greet_person<'a>(cx: Scope<'a, GreetPersonProps>) -> Element {
    let GreetPersonProps { person } = &cx.props;
    {
        render! {"hello, {person}"}
    }
}

// fn App(cx: Scope) -> Element {
//     let drawer_state = use_state(cx, || false);

//     let is_blue = use_state(cx, || true);

//     let color = if *is_blue.get() { "blue" } else { "red" };

//     // log::info!("Some info");
//     // log::error!("Error message");

//     cx.render(rsx! {
//         div {
//             div {
//                 key: "drawer",
//                 class: "drawer",
//                 background_color: "{color}",
//                 height: "150px",
//                 width: "150px",
//                 draggable: "true",

//                 ontouchstart: move |event| {
//                     is_blue.set(!is_blue);
//                     let x = event.data;
//                     log::info!("OnTouchStart: {:#?}", x);
//                 },
//                 ontouchmove: move |event| {
//                     let x = event.data;
//                     log::info!("OnTouchMove: {:#?}", x);
//                 },
//                 ontouchend: move |event| {
//                     let x = event.data;
//                     log::info!("OnTouchEnd: {:#?}", x);
//                 },

//                 p { "This is the drawer" }
//             }
//             div { key: "content", class: "content", p { "This is the main content" } }
//         }
//     })
// }
