use leptos::prelude::*;
use leptos::logging::log;
use std::error::Error;
use super::skill_view::{SkillList, StatsView};
use super::resource_views::HealthView;
use std::fs::read_to_string;
use std::path::Path;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::char::Character;
use crate::gear::GearData;
use crate::gear_views::{ArmorSelectionView, GearView};
use crate::resource_views::{ArmorView, HealthAdjustPopup};
use crate::shop_modal_view::{ShopModalData, ShopModalView};

pub fn read_gear_data_from_file<P: AsRef<Path>>(path: P) -> Result<GearData, Box<dyn Error>> {

    // Open the file in read-only mode with buffer.
    let check_file_path_result = std::fs::exists(&path);
    match check_file_path_result {
        Ok(exists) => {
            if exists {
                log!("Filepath exists");
                let file_str = read_to_string(&path)?;
                let lesson_data: GearData = serde_json::from_str(&file_str)?;
                return Ok(lesson_data);
            }
            else {
                log!("Filepath does not exist");
                return Err(Box::from("Filepath does not exist".to_string()));
            }
        },
        Err(error) => {
            let errorstring = format!("There was an issue locating the path, this might be due to accessing rights. Cannot confirm or deny existence:\n{error}");
            log!("{errorstring}");
            return Err(Box::from(error));
        },
    }
    

}


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
pub async fn get_all_data() -> Result<(Character, GearData), ServerFnError> {
    let read_data_result = read_character_data_from_file("./character.json");
    let gear_data_result = read_gear_data_from_file("./gear/final_dict.json");
    if read_data_result.is_ok() && gear_data_result.is_ok() {
        return Ok((read_data_result.unwrap(), gear_data_result.unwrap()))
    }
    else if read_data_result.is_err() {
        Err(ServerFnError::new(format!("char read data: {}", read_data_result.unwrap_err().to_string())))
    }
    else {
        Err(ServerFnError::new(format!("gear read data: {}", gear_data_result.unwrap_err().to_string())))
    }
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
            <body oncontextmenu="return false;">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> AnyView {
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
    }.into_any()
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> AnyView {
    view! {
        <Await
            future=get_all_data()
            let:all_data_result
        >{
            let result_clone = all_data_result.clone();
            move || {
                match result_clone.clone() {
                    Ok((char_data, gear_data)) => {
                        let cloned_char_data: Character = char_data.clone();
                        let cloned_gear_data: GearData = gear_data.clone();
                        view!{
                            <CharacterView character_data=cloned_char_data gear_data=cloned_gear_data/>
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
    }.into_any()
}

#[component]
fn CharacterView(character_data: Character, gear_data: GearData) -> AnyView {
    let char_rw_signal = RwSignal::new(character_data);
    let unlocked_signal = RwSignal::new(false);
    let save_char_action = Action::new(move |_: &()| async move {
        let char_copy = char_rw_signal.get_untracked();
        let _ = set_char_data(char_copy.clone()).await;
    });

    provide_context(char_rw_signal);
    provide_context(gear_data);

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

    let shop_modal_signal = RwSignal::new(ShopModalData::default());
    let damage_popup_signal = RwSignal::new(false);

    view! {
        <ShopModalView data=shop_modal_signal/>
        <HealthView on:click=move|_| damage_popup_signal.update(|v| *v = !*v)/>
        <Show when=move||damage_popup_signal.get()>
            <HealthAdjustPopup visible_signal=damage_popup_signal/>
        </Show>
        <div class="base_div">
            <div class="first_row">
                <h1 class="name">{move || char_rw_signal.read().name.clone()} / {move || char_rw_signal.read().alias.clone()}</h1>
                <div class="head_body_armor">
                    <ArmorView head=true/> 
                    <ArmorView head=false/> 
                </div>
            </div>
            <div class="columns">
                <div class="left_column">
                    <div class="skill_buttons">
                        <button on:click=move|_| {unlocked_signal.update(|s| *s = !*s) }>
                            {move|| {
                                if unlocked_signal.get(){"UNLOCKED".to_string()} 
                                else {"LOCKED".to_string()}
                            }}
                        </button>
                        <button on:click=move|_| char_rw_signal.update(|c| c.flip_flag("filter_zeros"))>FILTER</button>
                        <button on:click=move|_| char_rw_signal.update(|c| c.flip_flag("group_by_stat"))>GROUP</button>
                    </div>
                    <div class="skill_list">
                        <SkillList unlocked_signal=unlocked_signal/>
                    </div>
                </div>
                <div class="center_div">
                    <div class="center_div_first_row">
                        <StatsView/>
                        <ArmorSelectionView/>
                    </div>
                    <GearView/>
                </div>
                <div class="right_div">
                    <img class="char_image" src="Matchbox.jpg"/>
                    <div class="flex_row justify_center">
                        <button on:click=move|_| shop_modal_signal.update(|data| data.show())>SHOP</button>
                        <span class="money">{move||char_rw_signal.read().money}</span>
                    </div>
                </div>
            </div>
        </div>
    }.into_any()
}

