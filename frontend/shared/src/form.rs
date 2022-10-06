use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

use cobul::Model;
use validator::Validate;
pub use yew::*;

use api::{
    Algorithm, ContentFilter, Credentials, FilterBy, Guesses, Image, OrderBy, Orientation, Points,
    Quiz, Round, Tag,
};

#[derive(Clone)]
pub struct UseFormHandle<T: Clone> {
    errors: Rc<HashMap<String, String>>,
    dirty: UseStateHandle<HashSet<&'static str>>,

    state: Rc<T>,
    change: Callback<Rc<T>>,
}

impl<T: Clone> Deref for UseFormHandle<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T: Clone + 'static> UseFormHandle<T> {
    pub fn error(&self, key: &str) -> Option<String> {
        self.errors.get(&key.to_owned()).filter(|_| self.dirty.contains(key)).cloned()
    }

    pub fn errors(&self) -> &HashMap<String, String> {
        &self.errors
    }

    pub fn change<F>(
        &self,
        mapper: impl Fn(&mut T) -> &mut F + 'static,
        key: &'static str,
    ) -> Callback<F> {
        let state = self.state.clone();
        let dirty = self.dirty.clone();

        self.change.reform(move |value| {
            let mut new = (*dirty).clone();
            new.insert(key);
            dirty.set(new);

            let mut new = (*state).clone();
            *mapper(&mut new) = value;
            Rc::new(new)
        })
    }
}

impl UseFormHandle<Quiz> {
    pub fn public(&self) -> Model<bool> {
        let input = self.change(|x| &mut x.public, "public");
        Model { value: self.public, input }
    }

    pub fn complete(&self) -> Model<bool> {
        let input = self.change(|x| &mut x.complete, "complete");
        Model { value: self.complete, input }
    }

    pub fn title(&self) -> Model<String> {
        let input = self.change(|x| &mut x.title, "title");
        Model { value: self.title.clone(), input }
    }

    pub fn description(&self) -> Model<String> {
        let input = self.change(|x| &mut x.description, "description");
        Model { value: self.description.clone(), input }
    }

    pub fn explanation(&self) -> Model<String> {
        let input = self.change(|x| &mut x.explanation, "explanation");
        Model { value: self.explanation.clone(), input }
    }

    pub fn image(&self) -> Model<Image> {
        let input = self.change(|x| &mut x.image, "image");
        Model { value: self.image.clone(), input }
    }
    pub fn tags(&self) -> Model<Vec<Tag>> {
        let input = self.change(|x| &mut x.tags, "tags");
        Model { value: self.tags.clone(), input }
    }
}

impl UseFormHandle<Credentials> {
    pub fn email(&self) -> Model<String> {
        let input = self.change(|x| &mut x.email, "email");
        Model { value: self.email.clone(), input }
    }

    pub fn password(&self) -> Model<String> {
        let input = self.change(|x| &mut x.password, "password");
        Model { value: self.password.clone(), input }
    }
}

impl UseFormHandle<Round> {
    pub fn answer(&self) -> Model<String> {
        let input = self.change(|x| &mut x.answer, "answer");
        Model { value: self.answer.clone(), input }
    }

    pub fn points(&self) -> Model<Points> {
        let input = self.change(|x| &mut x.points, "points");
        Model { value: self.points.clone(), input }
    }

    pub fn guesses(&self) -> Model<Guesses> {
        let input = self.change(|x| &mut x.guesses, "guesses");
        Model { value: self.guesses.clone(), input }
    }

    pub fn speed(&self) -> Model<u64> {
        let input = self.change(|x| &mut x.speed, "speed");
        Model { value: self.speed.clone(), input }
    }

    pub fn algorithm(&self) -> Model<Algorithm> {
        let input = self.change(|x| &mut x.algorithm, "algorithm");
        Model { value: self.algorithm.clone(), input }
    }

    pub fn image(&self) -> Model<Image> {
        let input = self.change(|x| &mut x.image, "image");
        Model { value: self.image.clone(), input }
    }
}

impl UseFormHandle<FilterBy> {
    pub fn query(&self) -> Model<String> {
        let input = self.change(|x| &mut x.query, "query");
        Model { value: self.query.clone(), input }
    }

    pub fn page(&self) -> Model<u64> {
        let input = self.change(|x| &mut x.page, "page");
        Model { value: self.page.clone(), input }
    }

    pub fn per_page(&self) -> Model<u64> {
        let input = self.change(|x| &mut x.per_page, "per_page");
        Model { value: self.per_page.clone(), input }
    }

    pub fn order_by(&self) -> Model<OrderBy> {
        let input = self.change(|x| &mut x.order_by, "order_by");
        Model { value: self.order_by.clone(), input }
    }

    pub fn content_filter(&self) -> Model<ContentFilter> {
        let input = self.change(|x| &mut x.content_filter, "content_filter");
        Model { value: self.content_filter.clone(), input }
    }

    pub fn orientation(&self) -> Model<Orientation> {
        let input = self.change(|x| &mut x.orientation, "orientation");
        Model { value: self.orientation.clone(), input }
    }
}

#[hook]
pub fn use_form<T: Clone + Validate>(state: Rc<T>, change: Callback<Rc<T>>) -> UseFormHandle<T> {
    let dirty = use_state(|| HashSet::new());
    let errors = match state.as_ref().validate() {
        Ok(_) => return UseFormHandle { dirty, errors: Rc::new(HashMap::new()), state, change },
        Err(errors) => errors,
    };

    // SAFETY: Here the assumption is made that none of the vectors in the map are empty.
    let errors = errors
        .field_errors()
        .into_iter()
        .map(|(field, vec)| (field.to_owned(), vec.first().unwrap().to_string()))
        .collect();

    UseFormHandle { dirty, errors: Rc::new(errors), state, change }
}
