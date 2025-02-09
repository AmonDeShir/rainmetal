extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ActionComponent)]
pub fn action_component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let snake_case_name = to_snake_case(&name.to_string());

    let gen = quote! {
        impl ActionComponent for #name {
            fn new<S: PlannerState>() -> Action<S> {
                Action::new(#snake_case_name)
            }

            fn get_state(&self) -> &ActionState {
                &self.0
            }
        }

        impl InsertableActionComponent for #name {
            fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity) {
                commands.entity(entity_to_insert_to).insert(self.clone());
            }

            fn remove(&self, commands: &mut Commands, entity_to_remove_from: Entity) {
                commands.entity(entity_to_remove_from).remove::<Self>();
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(PlannerState)]
pub fn planner_state_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let gen = quote! {
        impl PlannerState for #name {}
    };

    gen.into()
}

fn to_snake_case(s: &str) -> String {
    let mut chars = s.chars().peekable();
    let mut snake_case = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_uppercase() {
            if !snake_case.is_empty() {
                snake_case.push('_');
            }
            snake_case.extend(c.to_lowercase());
        } else {
            snake_case.push(c);
        }
        chars.next();
    }
    snake_case
}
