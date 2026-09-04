# tui-a11y

Experimental Linux (AT-SPI) accessibility tree support for terminal UIs, via
[AccessKit]. Started as a spike inside [Ratatui]'s workspace ([see the
discussion][ratatui/ratatui#2610]) and moved out here since it has no
direct dependency on Ratatui and works with any TUI framework.

## What this is

Terminal UI frameworks render characters at cell positions; they have no
concept of focus, selection, or semantic roles, and no retained widget
tree. This crate doesn't try to infer any of that from a rendered screen
buffer. Instead, the app declares a small, separate accessibility tree by
hand, alongside its normal rendering code, using [`TreeBuilder`], and this
crate serves it to AT-SPI clients (screen readers, and testing tools like
[Accerciser]) via [`A11y`].

Building that tree from scratch every frame gets repetitive fast, so the
crate ships adapters that turn plain data (item strings, a selected index)
into a `SubTree` ready to merge in with `TreeBuilder::subtree`:
`list_nodes`, `table_nodes`, `tabs_nodes`, `gauge_nodes`, `text_nodes`, and
`group_nodes` (for naming/nesting the others). They take data, not widget
types, so they work with a hand-rolled widget or any TUI framework, not
just Ratatui. For a widget type you own, `Accessible`/`StatefulAccessible`
give it a home for an `a11y_nodes` method built on top of the same
adapters.

## Try it

```sh
cargo run --example list       # selectable list, full round trip
cargo run --example dashboard  # tabs + table + gauge, adapters composed
```

## The `IsEnabled` gotcha

`A11y::new` only actually registers on the AT-SPI bus once the desktop
reports `org.a11y.Status.IsEnabled = true`. That happens automatically once
a screen reader like Orca is running, but is otherwise off by default on
most Linux desktops, including most CI runners and headless dev boxes.
There's no error and no log line when it's off; `A11y::update` just
quietly does nothing. If you're testing without a screen reader:

```sh
gsettings set org.gnome.desktop.interface toolkit-accessibility true
```

## Linux only

Every AT-SPI adapter talks to apps over D-Bus, which needs no OS window
handle. Windows (UIA) and macOS (NSAccessibility) both require binding to
a real window, which a terminal app doesn't have. See
[ratatui/ratatui#2610] for the discussion; this crate doesn't attempt
either.

## Known gaps

- `Action::Focus` has no AT-SPI counterpart in the current
  `accesskit_atspi_common` mapping. AT-SPI drives focus from the tree's
  `focus` field; it has no inbound "move focus here" request.
  Only `Action::Click` round-trips.
- Node identity (`node_id`) is a plain key hash the app supplies; nothing
  here validates that it stays stable across frames. Get it wrong and
  focus/selection tracking degrades silently. `TreeBuilder::build` does
  catch duplicate ids and a dangling focus id in debug builds.
  `gauge_nodes` reports its ratio as `0..=100`, clamped. accesskit's
  numeric-value fields are unitless, so the convention is documented but
  not enforced.
- The adapters cover list/table/tabs/gauge/text/group. Wiring
  `ratatui-widgets`' actual widget types (`impl Accessible for List`, ...)
  behind a feature flag is unstarted for now, an app builds subtrees
  from its own data (see the `list` and `dashboard` examples), which also
  means these adapters work outside Ratatui entirely.
- Scrollbar, barchart, sparkline, chart, and calendar have no adapter yet.
- Windows (UIA) and macOS (NSAccessibility) are unattempted. See "Linux
  only" above.

## License

MIT, see [LICENSE](LICENSE).

[Ratatui]: https://ratatui.rs
[AccessKit]: https://github.com/AccessKit/accesskit
[Accerciser]: https://gitlab.gnome.org/GNOME/accerciser
[ratatui/ratatui#2610]: https://github.com/ratatui/ratatui/issues/2610
