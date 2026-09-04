//! Experimental Linux accessibility support for terminal UIs, via [AT-SPI]
//! and [AccessKit].
//!
//! Terminal UI frameworks render characters at cell positions; they have no
//! concept of focus, selection, or semantic roles, and no retained widget
//! tree to derive them from. This crate doesn't try to infer any of that
//! from a rendered screen buffer. Instead, the app declares a small,
//! separate accessibility tree by hand see [`TreeBuilder`] and this crate
//! serves it over AT-SPI via [`A11y`].
//!
//! # Quick start
//!
//! ```no_run
//! use tui_a11y::{A11y, Role, TreeBuilder, node_id};
//!
//! let window_id = node_id("window");
//! let mut tree = TreeBuilder::new();
//! tree.node(window_id, Role::Window, "My app", []);
//! tree.root(window_id);
//!
//! let mut a11y = A11y::new(tree.build());
//!
//! // ...in your render loop, after every draw:
//! let mut tree = TreeBuilder::new();
//! tree.node(window_id, Role::Window, "My app", []);
//! tree.root(window_id);
//! a11y.update(tree.build());
//! ```
//!
//! # The `IsEnabled` gotcha
//!
//! [`A11y::new`] only actually registers on the AT-SPI bus once the desktop
//! reports `org.a11y.Status.IsEnabled = true` which happens  automatically
//! once a screen reader like Orca is running, but is otherwise off by default
//! on most Linux desktops. There is no error and no log line when it's off:
//! [`A11y::update`] just quietly does nothing.
//! first:
//!
//! ```sh
//! gsettings set org.gnome.desktop.interface toolkit-accessibility true
//! ```
//!
//! [AT-SPI]: https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/
//! [AccessKit]: https://github.com/AccessKit/accesskit

mod accessible;
mod adapt;
mod adapter;
mod builder;
mod id;
mod subtree;

pub use accessible::{Accessible, StatefulAccessible};
pub use accesskit::{Action, ActionRequest, Node, NodeId, Role, TreeId, TreeInfo, TreeUpdate};
pub use adapt::{
    gauge_nodes, group_nodes, items_nodes, list_nodes, table_nodes, tabs_nodes, text_nodes,
};
pub use adapter::A11y;
pub use builder::TreeBuilder;
pub use id::node_id;
pub use subtree::SubTree;
