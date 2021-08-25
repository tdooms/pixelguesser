#[macro_export]
macro_rules! bool_prop {
    ($x:ident, $y:literal) => {
        #[derive(
            std::clone::Clone,
            std::fmt::Debug,
            std::cmp::PartialEq,
            std::default::Default,
            std::marker::Copy,
        )]
        pub struct $x(bool);

        impl std::convert::Into<yew::Classes> for $x {
            fn into(self) -> yew::Classes {
                match self.0 {
                    true => yew::Classes::from($y),
                    false => yew::Classes::new(),
                }
            }
        }

        impl yew::html::IntoPropValue<$x> for bool {
            fn into_prop_value(self) -> $x {
                $x(self)
            }
        }
    };
}

bool_prop!(Active, "is-active");
bool_prop!(Rounded, "is-rounded");
bool_prop!(Boxed, "is-boxed");
bool_prop!(Toggle, "is-toggle");
bool_prop!(Fullwidth, "is-fullwidth");
bool_prop!(Hidden, "is-hidden");
bool_prop!(Outlined, "is-outlined");
bool_prop!(Inverted, "is-inverted");
bool_prop!(Light, "is-light");
bool_prop!(Loading, "is-loading");
bool_prop!(Disabled, "is-disabled");
bool_prop!(Selected, "is-selected");
bool_prop!(Addons, "has-addons");
bool_prop!(Bordered, "is-bordered");
bool_prop!(Striped, "is-striped");
bool_prop!(Narrow, "is-narrow");
bool_prop!(Hoverable, "is-hoverable");
bool_prop!(Delete, "is-delete");
bool_prop!(Spaced, "is-spaced");
bool_prop!(Checked, "is-checked");
bool_prop!(Expanded, "is-expanded");
bool_prop!(Grouped, "is-grouped");
bool_prop!(GroupedMultiline, "is-grouped-multiline");
bool_prop!(Static, "is-static");
bool_prop!(FixedSize, "has-fixed-size");
bool_prop!(Multiline, "is-multiline");
bool_prop!(Centered, "is-centered");
bool_prop!(VCentered, "is-vcentered");
bool_prop!(Gapless, "is-gapless");
bool_prop!(Vertical, "is-vertical");
bool_prop!(Dropdown, "has-dropdown");
bool_prop!(Transparent, "is-transparent");
bool_prop!(Shadow, "has-shadow");
