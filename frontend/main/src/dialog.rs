use cobul::*;
use yew::*;

#[function_component(PlayDialog)]
pub fn play_dialog() -> Html {
    html! {
        <Dialog>
            <DialogHeader>
                <DialogHeaderTitle>{"Play"}</DialogHeaderTitle>
            </DialogHeader>
            <DialogBody>
                <p>{"This is a dialog"}</p>
            </DialogBody>
            <DialogFooter>
                <Button>{"Cancel"}</Button>
                <Button>{"Save"}</Button>
            </DialogFooter>
        </Dialog>
    }
}
