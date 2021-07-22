use yew::prelude::*;
use yew_router::components::Link;

use api::Quiz;

use crate::globals::IMAGE_ENDPOINT;
use crate::route::Route;

pub fn quiz_card(quiz: &Quiz) -> Html {
    html! {
        <div class="card" style="height:100%;display:flex;flex-direction:column">
            <div class="card-image">
                <figure class="image is-3by2">
                    <img src={format!("http://{}/{}", IMAGE_ENDPOINT, quiz.image_url)}/>
                </figure>
            </div>
            <div class="card-content" style="height:100%">
                <div class="media">
                    <div class="media-content" style="overflow:hidden">
                        <p class="title is-4"> {&quiz.name} </p>
                        <p class="subtitle is-6"> {&quiz.creator} </p>
                    </div>
                </div>
                <div class="content">
                    {&quiz.description}
                </div>
            </div>
            <div class="card-footer">
                <Link<Route> classes={classes!("button", "is-success", "is-fullwidth", "square-top")} route={Route::Host{quiz_id: quiz.quiz_id}}>
                    <span class="icon"><i class="fas fa-play"></i></span> <strong>{"Play"}</strong>
                </Link<Route>>
            </div>
        </div>
    }
}
