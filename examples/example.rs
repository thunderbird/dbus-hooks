use ksni::{self, menu, tray};

fn main() {
    struct Foo;
    impl ksni::Tray for Foo {
        type Err = std::convert::Infallible;
        fn tray_properties() -> tray::Properties {
            tray::Properties {
                icon_name: "desktop".to_owned(),
                ..Default::default()
            }
        }
        fn menu() -> Vec<menu::MenuItem> {
            use menu::*;
            vec![
                SubMenu {
                    label: "a".into(),
                    submenu: vec![
                        SubMenu {
                            label: "a1".into(),
                            submenu: vec![StandardItem {
                                label: "a1.1".into(),
                                activate: Box::new(|| println!("a")),
                                ..Default::default()
                            }
                            .into()],
                            ..Default::default()
                        }
                        .into(),
                        StandardItem {
                            label: "a2".into(),
                            ..Default::default()
                        }
                        .into(),
                    ],
                    ..Default::default()
                }
                .into(),
                MenuItem::Sepatator,
                CheckmarkItem {
                    label: "b".into(),
                    ..Default::default()
                }
                .into(),
            ]
        }
    }

    ksni::run(Foo);
}
