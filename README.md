# dbus\_hooks

`dbus_hooks` is forked from an upstream crate called `ksni` and modified to fit Thunderbird's needs. You can find the [![upstream `ksni` documentation](https://docs.rs/ksni/badge.svg)](https://docs.rs/ksni)
[![MSRV](https://img.shields.io/badge/msrv-1.58.0-blue)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)

A Rust implementation of the KDE/freedesktop StatusNotifierItem specification.

This crate is used in the [Linux system tray icon](https://github.com/thunderbird/linux-sys-tray/).

## Example

```rust
use dbus_hooks;

#[derive(Debug)]
struct MyTray {
    selected_option: usize,
    checked: bool,
}

impl dbus_hooks::Tray for MyTray {
    fn icon_name(&self) -> String {
        "help-about".into()
    }
    fn title(&self) -> String {
        if self.checked { "CHECKED!" } else { "MyTray" }.into()
    }
    // NOTE: On some system trays, `id` is a required property to avoid unexpected behaviors
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn menu(&self) -> Vec<dbus_hooks::MenuItem<Self>> {
        use dbus_hooks::menu::*;
        vec![
            SubMenu {
                label: "a".into(),
                submenu: vec![
                    SubMenu {
                        label: "a1".into(),
                        submenu: vec![
                            StandardItem {
                                label: "a1.1".into(),
                                ..Default::default()
                            }
                            .into(),
                            StandardItem {
                                label: "a1.2".into(),
                                ..Default::default()
                            }
                            .into(),
                        ],
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
            MenuItem::Separator,
            RadioGroup {
                selected: self.selected_option,
                select: Box::new(|this: &mut Self, current| {
                    this.selected_option = current;
                }),
                options: vec![
                    RadioItem {
                        label: "Option 0".into(),
                        ..Default::default()
                    },
                    RadioItem {
                        label: "Option 1".into(),
                        ..Default::default()
                    },
                    RadioItem {
                        label: "Option 2".into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }
            .into(),
            CheckmarkItem {
                label: "Checkable".into(),
                checked: self.checked,
                activate: Box::new(|this: &mut Self| this.checked = !this.checked),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

fn main() {
    let service = dbus_hooks::TrayService::new(MyTray {
        selected_option: 0,
        checked: false,
    });
    let handle = service.handle();
    service.spawn();

    std::thread::sleep(std::time::Duration::from_secs(5));
    // We can modify the tray
    handle.update(|tray: &mut MyTray| {
        tray.checked = true;
    });
    // Run forever
    loop {
        std::thread::park();
    }
}
```

Will create a system tray like this:

![screenshot_of_example_in_gnome.png](examples/screenshot_of_example_in_gnome.png)

(In GNOME with AppIndicator extension)

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or distribute this software, either in source code form or as a compiled binary, for any purpose, commercial or non-commercial, and by any means.
