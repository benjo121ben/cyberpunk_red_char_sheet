use leptos::prelude::*;


#[component]
pub fn AmmoView(count: RwSignal<i32>) -> impl IntoView {
    let check_visibility = move |nr| {
        if count.get() >= nr { "hidden"  } else {"visible"}
    };

    view! {
        <svg id="eHRlrbPxCUi1" on:click=move|_| count.update(|val| *val = *val + 1) xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 300 300" shape-rendering="geometricPrecision" text-rendering="geometricPrecision" project-id="95c5d48746bb4f409673ec40a2ef35d9" export-id="cf33590d4c6f4f0cbbde14f1333dbd2b" cached="false">
            <ellipse rx="50" ry="50" transform="matrix(-3 0 0-3.000001 150 149.765748)" fill="#d2dbed" stroke-width="0"/>
            <ellipse class="ammo_bullet" rx="50" ry="50" transform="matrix(-2.396996 0 0-2.396989 150 149.765748)" fill="#1da8df" stroke-width="0"/>
            <path d="M150,114.307486v90.338036" transform="matrix(3 0 0 3.000001-300-328.792324)" fill="none" stroke="#d2dbed" stroke-width="5" stroke-linecap="round"/>
            <path d="M150,114.307486v90.338036" transform="matrix(0 3-3.000001 0 628.429671-300.362653)" fill="none" stroke="#d2dbed" stroke-width="5" stroke-linecap="round"/>
            <path d="M150,114.307486v90.338036" transform="matrix(2.12132 2.12132-2.121321 2.121321 170.102813-506.86157)" fill="none" stroke="#d2dbed" stroke-width="5" stroke-linecap="round"/>
            <path d="M150,114.307486v90.338036" transform="matrix(2.12132-2.12132 2.121321 2.121321-506.498917 129.534534)" fill="none" stroke="#d2dbed" stroke-width="5" stroke-linecap="round"/>
            <ellipse rx="50" ry="50" transform="matrix(-1.153015 0 0-1.15301 150 149.765748)" fill="#d2dbed" stroke-width="0"/>
            <path style:visibility=move|| {check_visibility(8)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(-3 0 0 3.000001 596.127444-332.332035)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(7)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(0-3 3.000001 0-325.377493 609.896205)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(6)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(0 3 3.000001 0-324.163531-302.890133)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(5)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(-3 0 0-3.000001 604.018196 629.678837)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(4)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(3 0 0-3.000001-300.376306 632.245137)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(3)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(0 3-3.000001 0 633.875226-304.98783)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(2)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(0-3-3.000001 0 629.019378 594.443883)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(1)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(3 0 0 3.000001-303.411211-329.382031)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
        </svg> 

    }
}

#[component]
pub fn HealthView() -> impl IntoView {
    view! {
        <div class="health_border">
            <div class="health_bar"></div>
        </div>    
    }
}
