use leptos::prelude::*;
use crate::components::atoms::typography::{H4_CLASS, NORMAL_CLASS};

#[component]
fn TH(children: Children) -> impl IntoView {
    return view!{
        <th class=H4_CLASS>
            {children()}
        </th>
    };
}

#[component]
fn TD(children: Children) -> impl IntoView {
    return view!{
        <th class=NORMAL_CLASS>
            {children()}
        </th>
    };
}

#[component]
fn TR(children: Children) -> impl IntoView {
    return view! {
        <tr class="even:bg-surface-50 dark:even:bg-surface-950 odd:bg-surface-100 dark:odd:bg-surface-900 
        hover:bg-surface-300 dark:hover:bg-surface-700">
            {children()}
        </tr>
    };
}

#[component]
pub fn Table() -> impl IntoView {
    return view!{
        <table>
            <thead> 
                <tr class="bg-surface-200 dark:bg-surface-800">  
                    <TH>Name</TH>      
                    <TH>Age</TH>
                    <TH>City</TH>
                </tr>
            </thead>
            <tbody> 
                <TR>  
                    <TD>John</TD>    
                    <TD>25</TD>
                    <TD>Cape Town</TD>
                </TR>
                <TR>
                    <TD>Sarah</TD>
                    <TD>30</TD>
                    <TD>Johannesburg</TD>
                </TR>
            </tbody>
        </table>
    };
}