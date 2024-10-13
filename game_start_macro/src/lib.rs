extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
///
/// # start_game
///
/// #Example
/// ```rust
/// use game_start_macro::start_game;
///
/// #[start_game(Genshin)]
/// fn main() {
///    println!("Hello, world!");
///  //panic!("Hello, world!");
/// }
/// ```
///
/// #Errors
///
/// This function will return an error if the game is not found in the registry.
/// This function will return an error if the game executable is not found.
///
/// #Panics
///
/// This function will return an PANIC if the game executable is not found.
///
/// #Safety
///
/// This function is not marked as unsafe, but it is not safe to use in a concurrent environment.
/// It is just for fun.
///
/// #Note
///
/// This function is only available on Windows.
/// And it is just for fun.
/// Wish you have a good time ^_^
///
/// #Platform-specific
///
/// This function is only available on Windows.
#[proc_macro_attribute]
pub fn start_game(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_string = attr.to_string();
    let binding = attr_string.as_str();
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let block = &input.block;
    let output = quote! {
        {
            let no_panic = std::panic::catch_unwind(move ||
                #block
            );
            match no_panic {
                Ok(result) => result,
                Err(_) => {
                    println!("Game Start!");
                    match #binding {
                        "Genshin" => {
                            match start_gaming::find_exe(start_gaming::Game::Genshin) {
                                Ok(_) => {
                                    println!("Genshin Game Start!");
                                },
                                Err(_) => {
                                    panic!("Genshin Game Not Found!");
                                },
                            };
                        },
                        "WutheringWaves" => {
                            match start_gaming::find_exe(start_gaming::Game::WutheringWaves) {
                                Ok(_) => {
                                    println!("Wuthering Waves Game Start!");
                                },
                                Err(_) => {
                                    panic!("Wuthering Waves Game Not Found!");
                                },
                            };
                        },
                        _ => {
                            println!("Game Start!");
                        },
                    };
                    panic!("启动!");
                },
            }
        }
    };
    input.block = syn::parse2(output).unwrap();
    quote! { #input }.into()
}
