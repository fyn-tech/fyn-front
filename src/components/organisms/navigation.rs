/* ------------------------------------------------------------------------------------------------
 * Fyn-Front: Modern CFD/CAE Web Interface
 * Copyright (C) 2025 Fyn-Front Authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 * ------------------------------------------------------------------------------------------------
 * filename: navigation.rs
 * description: Navigation organism component
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::components::atoms::layout::*;
use crate::components::atoms::typography::*;
use crate::domain::user_context::UserContext;

#[component]
pub fn Navigation() -> impl IntoView {
    let user_context =
        use_context::<RwSignal<Option<UserContext>>>().expect("User context should be provided");

    return view! {
      <header class="w-full bg-surface-800">
        <div class="max-w-6xl mx-auto px-8 py-4">
          <Stack horizontal={true} align={FlexAlign::Center} add_class={"justify-between".to_string()}>
            <Stack>
              <H1 color={FONT_DRK_CLR.to_string()}>"Fyn-Tech"</H1>
              <H4 color={FONT_DRK_CLR.to_string()}>"creativity leads inovation"</H4>
            </Stack>
            <Stack horizontal={true}>
              <A href={"/simulate".to_string()} text_class={H4_CLASS.to_string()}>"Simulate"</A>
              { move || {
                match user_context.get() {
                  Some(user) =>{
                      let first_initial = user
                          .first_name
                          .as_ref()
                          .and_then(|s| s.chars().next());
                      let last_initial = user
                          .last_name
                          .as_ref()
                          .and_then(|s| s.chars().next());
                  view! {
                      <H4 color={LINK_CLR.to_string()}>{
                        format!("{}{}", first_initial.unwrap_or('?'), last_initial.unwrap_or('?'))}
                      </H4>
                  }.into_any()},
                  None => view! {
                      <A href={"/register".to_string()} text_class={H4_CLASS.to_string()}>"Register"</A>
                      <A href={"/sign_in".to_string()} text_class={H4_CLASS.to_string()}>"Sign In"</A>
                  }.into_any()
                }
              }
            }
            </Stack>
          </Stack>
        </div>
      </header>
    };
}
