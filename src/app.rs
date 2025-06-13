use leptos::prelude::*;
use leptos::logging::log;
use std::error::Error;
use super::skill_view::{SkillList, StatsView};
use super::resource_views::{HealthView, MoneyView};
use crate::add_skill_modal_view::AddSkillModalView;
use crate::app::server_fn::codec::Json;
use crate::info_modal_view::{SimpleModalData, SimpleModalView};
use crate::text_views::TextCenterSection;
use std::fs::read_to_string;
use std::path::Path;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use cp_char_data::char::Character;
use cp_char_data::gear::{GearData, RangeType};
use crate::gear_views::{ArmorSelectionView, GearView, RangeTable};
use crate::resource_views::{CurrentArmorView, HealthAdjustPopup, HumanityView, IPView};
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

pub fn check_password_file(pass: String) -> Result<(),  Box<dyn Error>> {
    let exists = std::fs::exists("passw.txt")?;
    let ret_val = if exists {
        log!("Filepath exists");
        let file_str = read_to_string("passw.txt").expect("reading to go without issue").replace("\n", "").trim().to_string();
        if file_str == pass {
            Ok(())
        }
        else {
            Err("password incorrect".to_string())
        }
    } else {
        Err("no passw file found".to_string())
    };

    ret_val.or_else(|errorstring|{
        log!("{errorstring}");
        return Err(Box::from(errorstring));
    })

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

#[server(name=SetCharData, prefix="/api", input=Json, output=Json, endpoint="set_char_data")]
pub async fn set_char_data(char: Character) -> Result<i32, ServerFnError> {
    let result = write_char_to_file("./character.json", &char);
    match result {
        Ok(_) => Ok(0),
        Err(error) => Err(ServerFnError::new(error)),
    }
}

#[server(name=CheckPassword, prefix="/api", input=Json, output=Json, endpoint="check_password")]
pub async fn check_password(passw: String) -> Result<(), ServerFnError> {
    let result = check_password_file(passw);
    match result {
        Ok(_) => Ok(()),
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
        <Stylesheet id="leptos" href="/pkg/cp_red_char_sheet.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=Login/>
                </Routes>
            </main>
        </Router>
    }.into_any()
}

#[component]
fn Login() -> AnyView {
    let show_main_page = RwSignal::new(false);
    let action_error = RwSignal::new("".to_string());
    let try_login = Action::new(move |password: &String| {
        let pw_clone = password.clone();
        async move {
            let result = check_password(pw_clone).await;
            if result.is_ok() {
                show_main_page.set(true);
            }
        }
    });

    view! {
        <Show when=move|| !show_main_page.get()>
            <div class="login_div">
                <input type="password"
                    value=""
                    prop:value=move|| {let _ = show_main_page(); "".to_string()}
                    on:change=move|ev| {
                        try_login.dispatch(event_target_value(&ev));
                    }
                />
            </div>
        </Show>
        <Show when=move|| {action_error.get() != "".to_string()}>
            <span>{move || action_error.get()}</span>
        </Show>
        <Show when=move|| show_main_page.get()>
            <HomePage/>
        </Show>
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

    let weapon_range_table_signal = RwSignal::new(RangeType::None);

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
    let simple_modal_signal = RwSignal::new(SimpleModalData::default());
    let show_add_skill_modal_signal = RwSignal::new(false);
    let damage_popup_signal = RwSignal::new(false);

    let danger_zone_memo = Memo::new(move|_| {
        char_rw_signal.read().get_wounded_action_penalty() >= 2
    });

    provide_context(char_rw_signal);
    provide_context(weapon_range_table_signal);
    provide_context(gear_data);
    provide_context(simple_modal_signal);

    view! {
        <div class="root_div"
            class:danger_zone=move ||danger_zone_memo.get()
        >
            <ShopModalView data=shop_modal_signal/>
            <Show when=move||show_add_skill_modal_signal.get()>
                <AddSkillModalView visible=show_add_skill_modal_signal/>
            </Show>
            <SimpleModalView data=simple_modal_signal/>
            <Show when=move||damage_popup_signal.get()>
                <HealthAdjustPopup visible_signal=damage_popup_signal/>
            </Show>
            <div class="base_div">
                <div class="first_row">
                    <h1 class="name">{move || char_rw_signal.read().name.clone()} / {move || char_rw_signal.read().alias.clone()}</h1>
                    <div class="health_and_armor_section">
                            <HealthView reverse=true on:click=move|_| damage_popup_signal.update(|v| *v = !*v)/>
                        <div class="head_body_armor">
                            <CurrentArmorView head=true/> 
                            <CurrentArmorView head=false/> 
                        </div>
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
                            <Show when=move|| unlocked_signal.get()>
                                <button on:click=move|_| {show_add_skill_modal_signal.set(true) }>+</button>
                            </Show>
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
                        <div class="center_split">
                            <GearView/>
                            <div class="flex_col">
                                <TextCenterSection/>
                                <RangeTable/>
                            </div>
                        </div>
                    </div>
                    <div class="right_div">
                        <div class="image_wrapper">
                            <img class="char_image" src="Matchbox.jpg"/>
                        </div>
                        <div class="edit_char_options">
                            <div class="flex_row justify_center">
                                <button on:click=move|_| shop_modal_signal.update(|data| data.show())>SHOP</button>
                                <MoneyView/>
                            </div>
                            <IPView/>
                            <HumanityView/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }.into_any()
}

