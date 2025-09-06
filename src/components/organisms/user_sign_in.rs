use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::section::*;

pub fn UserSignIn() -> impl IntoView {
    return view! {
      <form>
        <Section level={SectionLevel::H2} centre={true} spaced={true} title={"Sign In".to_string()}>
          <Grid size={Size::Xl} cols=2>
            <FormField
              label={"Username".to_string()}
              key={"username".to_string()}
              placeholder={"username".to_string()}
              input_type=InputType::Text { value: (None) }/>
            <FormField
              label={"Password".to_string()}
              key={"password".to_string()}
              input_type=InputType::Password/>
          </Grid>
        </Section>
        <Stack align=FlexAlign::Center>
          <Button text="Sign In".to_string()/>
        </Stack>
      </form>
    };
}
