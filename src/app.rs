use leptos::prelude::*;
use leptos::logging::log;
use std::{clone, error::Error};
use std::fs::read_to_string;
use std::path::Path;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::char::Character;

pub fn read_character_data_from_file<P: AsRef<Path>>(path: P) -> Result<Character, Box<dyn Error>> {

    // Open the file in read-only mode with buffer.
    let check_file_path_result = std::fs::exists(&path);
    match check_file_path_result {
        Ok(exists) => {
            if exists {
                log!("Filepath exists");
                let file_str = read_to_string(&path)?;
                let lesson_data: Character = serde_json::from_str(&file_str)?;
                return Ok(lesson_data);
            }
            else {
                log!("Filepath does not exist");
                return Ok(Character::zero());
            }
        },
        Err(error) => {
            let errorstring = format!("There was an issue locating the path, this might be due to accessing rights. Cannot confirm or deny existence:\n{error}");
            log!("{errorstring}");
            return Err(Box::from(error));
        },
    }
    

}

pub fn write_char_to_file<P: AsRef<Path>>(path: P, character: &Character) -> Result<(), Box<dyn Error>>{
    match serde_json::to_string_pretty(&character) {
        Ok(json) => {
            match std::fs::write(path, json) {
                Ok(_) => {
                    return Ok(());
                },
                Err(error) => {
                    println!("Error occurred during File writing: {error}");
                    return Err(Box::new(error));
                },
            }
        },
        Err(error) => { 
            println!("Error occurred during Serialization {error}");
            return Err(Box::new(error));
        }
    };
}

#[server(GetCharData, "/api", "GetJson", "get_char_data")]
pub async fn get_char_data() -> Result<Character, ServerFnError> {
    let read_data_result = read_character_data_from_file("./character.json");
    read_data_result.or_else(|error|{
        Err(ServerFnError::new(error.to_string()))
    })
}

#[server(SetCharData, "/api", "Url", "set_char_data")]
pub async fn set_char_data(char: Character) -> Result<i32, ServerFnError> {
    let result = write_char_to_file("./character.json", &char);
    match result {
        Ok(_) => Ok(0),
        Err(error) => Err(ServerFnError::new(error)),
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/test-leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Await
            future=get_char_data()
            let:char_data_result
        >{
            let result_clone = char_data_result.clone();
            move || {
                log!("{:#?}",result_clone.clone());
                match result_clone.clone() {
                    Ok(char_data) => {
                        let cloned_data: Character = char_data.clone();
                        view!{
                            <CharacterView character_data=cloned_data/>
                        }.into_any()
                    },
                    Err(error) => {
                        view! {
                            <p>{format!("there was an error loading the data: {0}", error.to_string())}</p>
                        }.into_any()
                    },
                }
                
            }
        }</Await>
    }
}

#[component]
fn CharacterView(character_data: Character) -> impl IntoView {
    let char_rw_signal = RwSignal::new(character_data);
    let save_char_action = Action::new(move |_: &()| async move {
        let char_copy = char_rw_signal.get_untracked();
        let _ = set_char_data(char_copy.clone()).await;
    });

    Effect::new(move |prev| {
        let _ = char_rw_signal.get();
        match prev {
            Some(_) => {
                save_char_action.dispatch(());
                return 0;
            },
            None => 0
        }
    });

    view! {
        <div class="base_div">
            <button on:click=move|_| { save_char_action.dispatch(()); }>TEST</button>
            <div class="columns">
                <div class="skill_list"></div>
                <div class="center_div"></div>
                <div class="combat_div"></div>
            </div>
        </div>
    }
}