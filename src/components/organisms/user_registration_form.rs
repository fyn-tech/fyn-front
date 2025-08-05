use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::section::*;

pub fn UserRegisterForm() -> impl IntoView {
    return view! {
      <form>
        <Section level={SectionLevel::H2} centre={true} spaced={true} title={"Register".to_string()}>
          <Grid size={Size::Xl} cols=2>
            <FormField
              label={"First Name".to_string()}
              key={"first_name".to_string()}
              input_type=InputType::Text { value: (None) }/>
            <FormField
              label={"Last Name".to_string()}
              key={"last_name".to_string()}
              input_type=InputType::Text { value: (None) }/>
            <FormField
              label={"Email".to_string()}
              key={"email".to_string()}
              input_type=InputType::Email/>
            <FormField
              label={"Password".to_string()}
              key={"password".to_string()}
              input_type=InputType::Password/>
            <FormField
              label={"Company".to_string()}
              key={"company".to_string()}
              input_type=InputType::Text { value: (None) }/>
            <FormField
              label={"Country".to_string()}
              key={"country".to_string()}
              input_type=InputType::Text { value: (None) }/>
          </Grid>
        </Section>
        <Stack align=FlexAlign::Center>
          <Button text="Create Account".to_string()/>
        </Stack>
      </form>
    };
}
