
use leptos::prelude::*;

#[component]
pub fn Stack(children: Children) -> impl IntoView {
    
    let class_str:String = "space-y-2".to_string();
    return view!{
        <div class={class_str}>
            {children()}
        </div>
    };
}