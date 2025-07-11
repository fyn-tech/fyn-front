
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing{
    Sm, // 8 (default)
    Md, // 16
    Lg, // 24
}

fn space_to_string(space: Spacing) -> &'static str{
    return match space {
        Spacing::Sm => "2",
        Spacing::Md => "3",
        Spacing::Lg => "4"
    };
}

#[component]
pub fn Stack(
    #[prop(default = Spacing::Sm)] space: Spacing,
    #[prop(default = false)] horizontal: bool,
    children: Children
) -> impl IntoView {
    
    let class_str:String = format!("flex flex-{} gap-{}", if horizontal {"row"} else {"col"}, space_to_string(space)).to_string();
    return view!{
        <div class={class_str}>
            {children()}
        </div>
    };
}