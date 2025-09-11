use leptos::prelude::*;
use leptos::reactive::signal;

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::molecules::button_bar::*;
use crate::components::molecules::table::*;
use crate::components::organisms::job_config_form::*;
use crate::components::organisms::navigation::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimulateView {
    FormAndViewer,
    TableView,
}

/// Simulate Page - Left toolbar template with dynamic content area
#[component]
pub fn Simulate() -> impl IntoView {
    let (current_view, set_current_view) = signal(SimulateView::FormAndViewer);

    view! {
        <Navigation/>

        <div class="h-screen w-full flex bg-surface-50 dark:bg-surface-950">
            <ButtonBar horizontal=false items = vec![
                view! {<GroupButton text="SM".to_string() size=Size::Md on_click=Box::new(move || {set_current_view.set(SimulateView::FormAndViewer);})/>},
                view! {<GroupButton text="RS".to_string() size=Size::Md on_click=Box::new(move || {set_current_view.set(SimulateView::TableView);})/>},
            ] />

            // Main content area - displays different components based on toolbar selection
            <div class="flex-1 flex flex-col">
                {move || match current_view.get() {
                    SimulateView::FormAndViewer => view! { <FormAndViewerLayout /> }.into_any(),

                    SimulateView::TableView => view! { <Table table={
                    TableStruct {name : "Table Name".to_string(),
                                    data: TableData{
                                    col_def: vec![
                                        ColumnDefinition {
                                        name: "Column 1".to_string(),
                                        data_type: CellType::Text
                                        }],
                                    rows: vec![
                                        vec!["text 0".to_string()],
                                        vec!["text 1".to_string()],
                                    ]
                                    } }}></Table>
                                    }.into_any(),

                }}
            </div>
        </div>
    }
}

/// Form and 3D Viewer Layout Component with working resizable splitter
#[component]
fn FormAndViewerLayout() -> impl IntoView {
    let (splitter_x, set_splitter_x) = signal(400.0);
    let (is_dragging, set_is_dragging) = signal(false);

    view! {
        <div class="h-full relative">
            // Form section - width controlled by splitter position
            <div
                class="absolute top-0 left-0 h-full bg-white dark:bg-surface-800 border-r border-surface-200 dark:border-surface-700 overflow-hidden"
                style:width=move || format!("{}px", splitter_x.get())
            >
                <JobConfigForm />
            </div>

            // Draggable vertical splitter line
            <div
                class=move || format!(
                    "absolute top-0 h-full w-2 cursor-col-resize z-10 {}",
                    if is_dragging.get() {
                        "bg-blue-500"
                    } else {
                        "bg-gray-400 hover:bg-blue-400"
                    }
                )
                style:left=move || format!("{}px", splitter_x.get())
                on:mousedown=move |e| {
                    e.prevent_default();
                    set_is_dragging.set(true);
                }
            >
                // Visual grip
                <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-1 h-8 bg-white rounded shadow"></div>
            </div>

            // 3D Viewer section - starts after the splitter
            <PlaceholderRender splitter_x = {splitter_x} />

            // Global mouse handlers for smooth dragging
            {move || if is_dragging.get() {
                view! {
                    <div
                        class="fixed inset-0 z-50 cursor-col-resize"
                        on:mousemove=move |e| {
                            if is_dragging.get() {
                                let new_x = e.client_x() as f64;
                                let clamped_x = new_x.max(100.0).min(1600.0); // Min/max limits
                                set_splitter_x.set(clamped_x);
                            }
                        }
                        on:mouseup=move |_| {
                            set_is_dragging.set(false);
                        }
                    ></div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}
        </div>
    }
}

#[component]
fn PlaceholderRender(splitter_x: ReadSignal<f64>) -> impl IntoView {
    return view! {
        <div
            class="absolute top-0 h-full bg-surface-100 dark:bg-surface-900"
            style:left=move || format!("{}px", splitter_x.get() + 8.0)
            style:right="0"
        >
            <div class="flex items-center justify-center h-full">
                <div class="text-center">
                    <div class="text-6xl mb-4">"🎲"</div>
                    <h3 class="text-xl font-semibold text-surface-900 dark:text-surface-100">"3D Viewer"</h3>
                    <p class="text-surface-600 dark:text-surface-400 mt-2">
                        "Splitter at: " {move || format!("{:.0}px", splitter_x.get())}
                    </p>
                    <p class="text-xs text-surface-500 dark:text-surface-400 mt-1">
                        "Drag the vertical line to resize"
                    </p>
                </div>
            </div>
        </div>
    };
}
