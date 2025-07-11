
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

#[component]
pub fn Grid(
    #[prop(default = Spacing::Sm)] space: Spacing,
    #[prop(default = 0)] cols: u8,
    #[prop(default = 0)] rows: u8,
    children: Children) -> impl IntoView{

    let col_str = if cols != 0 {format!("grid-cols-{}", cols)} else {"".to_string()};
    let row_str = if rows != 0 {format!("grid-rows-{}", rows)} else {"".to_string()};
    let class_str:String = format!("grid {} {} gap-{}", col_str, row_str, space_to_string(space)).to_string();
    return view!{
        <div class={class_str}>
            {children()}
        </div>
    };
}
