use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::size::*;
use crate::components::atoms::layout::{spacing, Align, BorderColor, BorderedDiv};
use crate::components::atoms::typography::{H3, H4_CLASS, NORMAL_CLASS};

// ------------------------------------------------------------------------------------------------
//  Data Structs
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CellType {
    // we can get more complicated later.
    Text,
    Float,
    Int,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStruct {
    pub name: String,
    pub data: TableData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub col_def: Vec<ColumnDefinition>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: CellType,
}

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

fn cell_format() -> String {
    return format!(
        "px-{} py-{} border-r {} last:border-r-0 ",
        spacing(Size::Sm),
        spacing(Size::Sm),
        BorderColor::Surface
    );
}

#[component]
fn TH(children: Children) -> impl IntoView {
    return view! {
        <th class=format!("{} {} {}", H4_CLASS, cell_format(), Align::Left)>
            {children()}
        </th>
    };
}

#[component]
fn TD(cell_type: CellType, children: Children) -> impl IntoView {
    return view! {
        <td class=format!("{} {} {}", NORMAL_CLASS, cell_format(),
                        if cell_type == CellType::Text {Align::Left} else {Align::Right})>
            {children()}
        </td>
    };
}

#[component]
fn TR(children: Children) -> impl IntoView {
    return view! {
        <tr class="
            even:bg-surface-50 dark:even:bg-surface-950 
            odd:bg-surface-100 dark:odd:bg-surface-900 
            hover:bg-surface-300 dark:hover:bg-surface-700
        ">
            {children()}
        </tr>
    };
}

#[component]
pub fn Table(table: TableStruct) -> impl IntoView {
    // title
    let title = if !table.name.is_empty() {
        Some(view! {<H3 align=Align::Center>{table.name}</H3>})
    } else {
        None
    };

    // Headers and Data
    let col_defs = table.data.col_def;
    let rows = table.data.rows;
    let column_headers: Vec<_> = col_defs
        .clone()
        .into_iter()
        .map(|col_def| view! {<TH>{col_def.name.clone()}</TH>})
        .collect();
    let row_data: Vec<_> = rows
        .into_iter()
        .map(|row_cells| {
            let cells: Vec<_> = row_cells
                .into_iter()
                .zip(col_defs.iter()) // Now col_defs is available
                .map(
                    |(col_data, col_def)| view! {<TD cell_type={col_def.data_type}>{col_data}</TD>},
                )
                .collect();
            view! {<TR>{cells}</TR>}
        })
        .collect();

    return view! {

        {title}
        <BorderedDiv>
            <table class="w-full border-collapse">
                <thead>
                    <tr class="bg-surface-200 dark:bg-surface-800">
                        {column_headers}
                    </tr>
                </thead>
                <tbody>
                    {row_data}
                </tbody>
            </table>
        </BorderedDiv>
    };
}
