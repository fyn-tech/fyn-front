use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::section::*;
use crate::infrastructure::fyn_api_client;
use crate::infrastructure::fyn_api_client::FynApiClient;

pub fn UserSignIn() -> impl IntoView {
    let fyn_api_client = use_context::<FynApiClient>().expect("FynApiClient should be provided");

    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let handle_login = move || {
        let username_str = username.get();
        let password_str = password.get();

        if username_str.is_empty() || password_str.is_empty() {
            leptos::logging::log!("Username and password required");
            return;
        }

        fyn_api_client.login(username_str, password_str);
    };

    return view! {
      <form>
        <Section level={SectionLevel::H2} centre={true} spaced={true} title={"Sign In".to_string()}>
          <Grid size={Size::Xl} cols=2>
            <FormField
              label={"Username".to_string()}
              key={"username".to_string()}
              placeholder={"username".to_string()}
              input_type=InputType::Text { signal: username }/>
            <FormField
              label={"Password".to_string()}
              key={"password".to_string()}
              input_type=InputType::Password {signal: password}/>
          </Grid>
        </Section>
        <Stack align=FlexAlign::Center>
          <Button text="Sign In".to_string() on_click=Box::new(handle_login)/>
        </Stack>
      </form>
    };
}
