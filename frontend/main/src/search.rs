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
    pub sort: Model<Sort>,
    pub filter: Model<String>,
}

#[function_component(Search)]
pub fn search(props: &Props) -> Html {
    let Props { sort, filter } = props.clone();

    html! {
        <Columns centered=true class="py-3">
        <Column size={ColumnSize::Is8}>

        <Field grouped=true>
            <Control expanded=true>
                // <Input placeholder="Find a quiz" model={filter} />
            </Control>
            <Control>
                // <simple::Dropdown<Sort> model={sort} />
            </Control>
        </Field>

        </Column>
        </Columns>
    }
}
