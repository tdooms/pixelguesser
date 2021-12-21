// use cobul::{props::Alignment, *};
// use futures::FutureExt;
// use gloo::file::callbacks::FileReader;
// use gloo::file::futures::read_as_data_url;
// use gloo::file::FileReadError;
// use photon_rs::monochrome::grayscale_human_corrected;
// use photon_rs::PhotonImage;
// use wasm_bindgen_futures::spawn_local;
// use yew::prelude::*;
//
// #[derive(Default)]
// pub struct Test {
//     image: Option<PhotonImage>,
//     reader: Option<FileReader>,
// }
//
// pub enum Msg {
//     Upload(Vec<web_sys::File>),
//     Read(Result<PhotonImage, FileReadError>),
// }
//
// async fn read_image(file: web_sys::File) -> Msg {
//     let base64 = match read_as_data_url(&file.into()).await {
//         Ok(ok) => ok,
//         Err(err) => return Msg::Read(Err(err)),
//     };
//
//     let stripped = base64.split(',').nth(1).unwrap();
//     let img = PhotonImage::new_from_base64(&stripped);
//
//     Msg::Read(Ok(img))
// }
//
// impl Component for Test {
//     type Message = Msg;
//     type Properties = ();
//
//     fn create(ctx: &Context<Self>) -> Self {
//         Self::default()
//     }
//
//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Upload(files) if files.len() == 1 => {
//                 let file = files.first().unwrap().clone();
//                 ctx.link().send_future(read_image(file));
//             }
//             Msg::Upload(..) => {
//                 log::error!("multiple files")
//             }
//             Msg::Read(Ok(mut img)) => {
//                 log::info!("hello imag");
//                 grayscale_human_corrected(&mut img);
//                 self.image = Some(img);
//             }
//             Msg::Read(Err(err)) => {
//                 log::error!("{:?}", err)
//             }
//         }
//
//         true
//     }
//
//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let sidebar = html! {
//             <>
//             <hr>
//             <File boxed=true alignment={Alignment::Centered} onupload={ctx.link().callback(Msg::Upload)} />
//             <Button><Icon icon=Icons::Undo/><span> {"Undo"} </span> </Button>
//             </>
//         };
//
//         let center = html! {
//             <Center>
//                 <DynImage src={self.image.as_ref().map(|img| img.get_base64())} />
//             </Center>
//         };
//
//         html! {
//             <Columns>
//                 <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} extra="p-0" overflow=false footer={left_footer}>
//                     {sidebar}
//                 </Sidebar>
//
//                 <Column size={ColumnSize::Is8}>
//                     { center }
//                 </Column>
//             </Columns>
//         }
//     }
// }
