use cobul::*;
use strum::EnumIter;
use yew::Callback;
use yew::*;

#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, EnumIter)]
pub enum Sort {
    Relevance,
    Difficulty,
    Popularity,
    Rating,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub sort: Sort,
    pub filter: String,

    pub onsort: Callback<Sort>,
    pub onfilter: Callback<String>,
}

#[function_component(Search)]
pub fn search(props: &Props) -> Html {
    let Props { sort, filter, onsort, onfilter } = props.clone();

    html! {
        <Columns centered=true class="py-4">
        <Column size={ColumnSize::Is8}>

        <Field grouped=true>
            <Control expanded=true>
                <Input placeholder="Find a quiz" value={filter} oninput={onfilter}/>
            </Control>
            <Control>
                <EnumDropdown<Sort> value={sort} onchange={onsort}/>
            </Control>
        </Field>

        </Column>
        </Columns>
    }
}
