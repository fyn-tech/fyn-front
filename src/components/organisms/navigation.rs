use leptos::prelude::*;

use crate::components::atoms::layout::*;
use crate::components::atoms::typography::*;

pub fn Navigation() -> impl IntoView {
    return view! {
      <header class="w-full bg-surface-800">
        <div class="max-w-6xl mx-auto px-8 py-4">
          <Stack horizontal={true} align={FlexAlign::Center} add_class={"justify-between".to_string()}>
            <Stack>
              <H1 color={FONT_DRK_CLR.to_string()}>"Fyn-Tech"</H1>
              <H4 color={FONT_DRK_CLR.to_string()}>"creativity leads inovation"</H4>
            </Stack>
            <Stack horizontal={true}>
              <A href={"/register".to_string()} text_class={H4_CLASS.to_string()}>"Register"</A>
              <A href={"/sign_in".to_string()} text_class={H4_CLASS.to_string()}>"Sign In"</A>
            </Stack>
          </Stack>
        </div>
      </header>
    };
}
