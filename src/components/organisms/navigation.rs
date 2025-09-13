use leptos::prelude::*;

use crate::components::atoms::layout::*;
use crate::components::atoms::typography::*;
use crate::domain::user_context::UserContext;

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
