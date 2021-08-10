// use crate::classify;
// use yew::prelude::*;
// use yewtil::NeqAssign;
//
// // use crate::elements::button::Button;
//
// #[derive(Clone, Debug, Properties, PartialEq)]
// pub struct DropdownProps {
//     /// The content of the dropdown menu.
//     ///
//     /// This content will be placed directly within the `div.dropdown-content` container.
//     #[prop_or_default]
//     pub children: Children,
//     #[prop_or_default]
//     pub extra: String,
//     /// Make this dropdown triggerable based on hover.
//     #[prop_or_default]
//     pub hoverable: bool,
//     /// Any additional classes to use for the trigger button.
//     #[prop_or_default]
//     pub button_extra: String,
//     /// The content of the trigger button.
//     #[prop_or_default]
//     pub button_html: Html,
// }
//
// /// An interactive dropdown menu for discoverable content.
// ///
// /// [https://bulma.io/documentation/components/dropdown/](https://bulma.io/documentation/components/dropdown/)
// pub struct Dropdown {
//     link: ComponentLink<Self>,
//     props: DropdownProps,
//     is_menu_active: bool,
// }
//
// impl Component for Dropdown {
//     type Message = ();
//     type Properties = DropdownProps;
//
//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self {
//             link,
//             props,
//             is_menu_active: false,
//         }
//     }
//
//     fn update(&mut self, _: Self::Message) -> ShouldRender {
//         if self.props.hoverable {
//             return false;
//         }
//         // Flip bool
//         self.is_menu_active ^= true;
//         true
//     }
//
//     fn change(&mut self, props: Self::Properties) -> ShouldRender {
//         self.props.neq_assign(props)
//     }
//
//     fn view(&self) -> Html {
//         // html! {<div onclick=self.link.callback(|_| DropdownMsg::Close) style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"></div>}
//
//         let DropdownProps { hoverable, .. } = self.props;
//         let active = self.is_menu_active;
//
//         html! {
//             <div class=classes!("dropdown", &self.props.extra, classify!(hoverable, active))>
//                 // {overlay}
//                 <div class="dropdown-trigger">
//                     <Button classes=self.props.button_extra.clone() onclick=self.link.callback(|_|())>
//                         { self.props.button_html.clone() }
//                     </Button>
//                 </div>
//                 <div class="dropdown-menu" role="menu">
//                     <div class="dropdown-content">
//                         { for self.props.children.iter() }
//                     </div>
//                 </div>
//             </div>
//         }
//     }
// }
