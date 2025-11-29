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
use crate::components::molecules::drop_down::*;
use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;

fn logout_user_update() {
    LocalResource::new(move || async move {
        let fyn_api_client =
            use_context::<FynApiClient>().expect("FynApiClient should be provided");

        let user_context = use_context::<RwSignal<Option<UserContext>>>()
            .expect("UserContext should be provided.");
        fyn_api_client.logout().await;
        user_context.set(None);
    });
}

#[component]
pub fn Navigation() -> impl IntoView {
    let user_context =
        use_context::<RwSignal<Option<UserContext>>>().expect("User context should be provided");

    let user_initials = Memo::new(move |_| {
        user_context
            .get()
            .map(|user| {
                let first = user
                    .first_name
                    .as_ref()
                    .and_then(|s| s.chars().next())
                    .unwrap_or('?');
                let last = user
                    .last_name
                    .as_ref()
                    .and_then(|s| s.chars().next())
                    .unwrap_or('?');
                format!("{}{}", first, last)
            })
            .unwrap_or_default()
    });

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
                  Some(_) => view! {
                    <DropDown trigger={view! {<H4 color={LINK_CLR.to_string()}>{user_initials.get()}</H4>}}>
                      <A href={"/user_preferences".to_string()} text_class={H4_CLASS.to_string()}>"Preference"</A>
                      <div
                          class="cursor-pointer"
                          on:click=move |_| {logout_user_update(); }
                      >
                      <A href={"/".to_string()} text_class={H4_CLASS.to_string()}>"Sign Out"</A>
                      </div>
                    </DropDown>
                  }.into_any(),
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
