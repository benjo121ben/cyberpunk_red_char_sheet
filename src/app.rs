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


/* #[component]
fn TimeTable(lesson_data: LessonData) -> impl IntoView {
    let today = chrono::offset::Local::now();
    let current_weekday = today.date_naive().weekday();
    let weekday_number = current_weekday.number_from_monday();
    let monday_date_opt = today.checked_sub_days(Days::new((weekday_number - 1).into()));
    if monday_date_opt.is_none() {
        return view! {
            <p>Could not aquire monday date</p>
        }.into_any()
    }
    let monday_date = monday_date_opt.expect("monday date should be unwrappable at this point");
    let lesson_data_signal = RwSignal::new(lesson_data);
    let week_add_nr: RwSignal<i32> = use_context().expect("week number should be set at this point");
    let monday_date_memo = Memo::new(move |_| {
        let week_modifier = week_add_nr.get();

        if week_modifier > 0 {
            let modifier = u64::try_from(week_modifier).unwrap();
            return monday_date.checked_add_days(Days::new(modifier * 7)).expect("date-week addition should exist");
        }
        else if week_modifier < 0 {
            let modifier = u64::try_from(week_modifier * -1).unwrap();
            return monday_date.checked_sub_days(Days::new(modifier * 7)).expect("date-week subtraction should exist");
        }

        return monday_date.clone();
    });
    provide_context(lesson_data_signal);
    
    let weekday_dates = move || {
        (0..5u64)
        .map(|number| monday_date_memo.get().checked_add_days(Days::new(number)).unwrap())
        //.map(|date| format!("{}/{}/{}", date.day(), date.month(), date.year()))
        .map(|date| date.format("%d/%m/%Y").to_string())
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, String)>>()
    };
    let mut times: Vec<String> = vec![];
    for offset in 0..13 {
        times.push(format!("{}:00", 7 + offset));
        times.push(format!("{}:30", 7 + offset));
    };
    let index_times: Vec<(usize, String)> = times.into_iter().enumerate().collect();
    let index_times_clone = index_times.clone();
    view!{
        <div class="time-table">
            <For
                each=move || index_times.clone()
                key=|(_, time)| {
                    time.to_string()
                }
                children=move |(idx, time)| {
                    view! {
                        <div
                            class="time-start"
                            style:grid-row=move || format!("{} / span 2", idx * 2 + 1)
                        >
                            {move || time.clone()}
                        </div>
                    }
                }
            />
            <For
                each=move || index_times_clone.clone()
                key=|(_, time)| {
                    time.to_string()
                }
                children=move |(idx, time)| {
                    view! {
                        <div
                            class="time-end"
                            style:grid-row=move || format!("{} / span 2", idx * 2 + 1)
                        >
                            {move || time.clone()}
                        </div>
                        <div 
                            class="background-row"
                            class:background-row-dark=move || idx % 2 == 0
                            class:background-row-bright=move || idx % 2 == 1
                            style:grid-row=move || format!("{} / span 2", idx * 2 + 2)
                        ></div>
                    }
                }
            />
            <For
                each=move || weekday_dates()
                key=|(_, date)| {
                    date.to_string()
                }
                children=move |(indx, date)| {
                    view! {
                        <DayEntry indx date/>
                    }
                }
            />
        </div>
    }.into_any()
    
}

#[component]
fn DayEntry(indx: usize, date: String) -> impl IntoView {
    let weekdays = vec!["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];
    let lesson_data: RwSignal<LessonData> = use_context().expect("Expecting Lesson data to exist");
    let date_clone = date.clone();
    let courses = move || lesson_data.with(|data| {
        let lesson_list_opt = data.lessons_list.get(&date_clone);
        if lesson_list_opt.is_none() {
            return vec![];
        }
        return lesson_list_opt.unwrap().clone();
    });
    view! {
        <div
            style:grid-column-start=move||{(indx + 2).to_string()}
            class="header"
        >
            {move || format!("{} {}", weekdays.get(indx).expect("day name should bet there").to_string(),  date.to_string())}
        </div>

        <For
            each=move || courses()
            key=|course| course.start.clone()
            children=move |course| {
                view! {
                    <CourseEntry course weekday=indx/>
                }
            }
        />
    }
}

#[component]
fn CourseEntry(course: Course, weekday: usize) -> impl IntoView {
    let COLORS: HashMap<&str, &str> = use_context().expect("expecting color hashmap");
    let (start_hour, start_minute) = get_hour_and_min_from_time(course.start);
    let (end_hour, end_minute) = get_hour_and_min_from_time(course.end);

    let start_y_offset = (start_hour - 7) * 4 + start_minute / 15; 
    let end_y_offset = (end_hour - 7) * 4 + end_minute / 15; 

    let color = COLORS.get(&course.name.clone() as &str).cloned().or(Some("#c23138")).unwrap();

    assert!(start_y_offset > 0);
    assert!(end_y_offset > 0);
    assert!(start_y_offset < end_y_offset);

    view! {
        <div
            class="table-entry"
            style:background-color=move || color
            style:grid-column-start=move || (weekday + 2).to_string()
            style:grid-row=move || format!("{} / span {}", start_y_offset + 2, end_y_offset - start_y_offset)
        >
            {move || course.name.clone()}
            <br/>
            {move || course.aula.clone()}
            <br/>
            {move || course.note.clone().or(Some("".to_string())).unwrap()}
        </div>
    }

}


fn get_hour_and_min_from_time(time: String) -> (i32, i32) {
    let split_time: Vec<String> = time.split(":").map(|val|val.to_string()).collect();
    let hour: i32 = split_time.get(0).unwrap().parse().unwrap();
    let minute: i32 = split_time.get(1).unwrap().parse().unwrap();
    return (hour, minute)
} */