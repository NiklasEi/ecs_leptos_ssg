mod components;

use components::navigation::Navigation;

use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{IntoSystem, Query};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::sync::{Arc, Mutex};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Meta name="description" content="A static website generated using Leptos and Bevy ECS"/>
        <Stylesheet href="/pkg/ecs_leptos_ssg.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes>
                    <StaticRoute
                        path="/"
                        view=HomePage
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(empty_static_params))),
                        )
                    />

                    <StaticRoute
                        path="/404"
                        view=NotFound
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(empty_static_params))),
                        )
                    />

                    <StaticRoute
                        path="/person/*person"
                        view=HomePage
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(people_static_params))),
                        )
                    />

                </Routes>
            </main>
        </Router>
    }
}

fn empty_static_params() -> StaticParamsMap {
    StaticParamsMap::default()
}

fn people_static_params(people: Query<&PersonId>) -> StaticParamsMap {
    let mut map = StaticParamsMap::default();
    map.insert(
        "person".to_string(),
        people.iter().map(|person| person.0.clone()).collect(),
    );
    map
}

#[derive(PartialEq, Params, Clone)]
struct PersonRouteParams {
    any: String,
}

#[derive(Resource, Clone)]
pub struct SiteName(pub String);

#[derive(Component, Clone)]
pub struct PersonName(pub String);

#[derive(Component, Clone)]
pub struct PersonId(pub String);

#[derive(Component)]
pub struct Age(pub u8);

#[component]
fn HomePage() -> impl IntoView {
    let route = use_route();
    let params = route.params().get();
    let no_person = "Dr. Who".to_owned();
    let current_person = params.0.get("person");
    let current_person = current_person.unwrap_or(&no_person);

    view! {
        <Navigation/>
        <h1>"Hello " {current_person} ", welcome to Leptos!"</h1>
        <Counter/>
    }
}

#[island]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { <button on:click=on_click>"Click Me: " {count}</button> }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <Navigation/>
        <h1>"Not Found from Leptos"</h1>
    }
}
