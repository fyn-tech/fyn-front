use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::section::*;
use crate::components::organisms::navigation::*;

fn RegisterForm() -> impl IntoView {
    return view! {
      <Section level={SectionLevel::H2} centre={true} spaced={true} title={"User Registration".to_string()}>
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
            label={"Username".to_string()}
            key={"user_name".to_string()}
            input_type=InputType::Text { value: (None) }/>
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
    };
}

fn Footer() -> impl IntoView {
    return view! {
      <footer class="w-full bg-surface-100 dark:bg-surface-900">
        <div class="max-w-6xl mx-auto px-8 py-4">

        </div>
      </footer>
    };
}

#[component]
pub fn Default(children: Children) -> impl IntoView {
    view! {
        <Stack
            horizontal=false
            fill_space=true
            add_class="min-h-screen justify-between".to_string()
        >

                    <Navigation/>


            <main class="flex-1 max-w-4xl mx-auto px-8 py-8 w-full">
                {children()}
            </main>

            <Footer/>
        </Stack>
    }
}

#[component]
pub fn Register() -> impl IntoView {
    return view! {
      <Default>
        <RegisterForm/>
      </Default>
    };
}
