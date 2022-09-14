use std::{convert::*, str::*};

use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use strum::IntoEnumIterator;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {

        <div class="container" style="display: flex; flex-direction: column;">

        <div style=" position: sticky; top: 0; padding: 40px; z-index:1;">
            <div style="display: flex; flex-direction: row;">
                <InputBox /> <Categories />
            </div>
            <ErrorBox />
        </div>

        <DisplayBox/>
        </div>
    }
}

#[function_component(InputBox)]
pub fn input_box() -> Html {
    let text = use_selector(|state: &FullState| state.text.clone())
        .as_ref()
        .clone();
    let oninput = Dispatch::<FullState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        let value = input.value();
        s.change_text(value);
    });

    html!(


        <div>
            <input type="text" id="textinput" name="input" placeholder="Search" value={text} {oninput}/>
        </div>
    )
}

#[function_component(ErrorBox)]
pub fn error_box() -> Html {
    let err = use_selector(|s: &FullState| s.warning.clone())
        .as_ref()
        .clone()
        .unwrap_or_else(|| "".to_string());

    if err == "" {
        html!(<> </>)
    } else {
        html!(<code> {err} </code>)
    }
}

#[function_component(DisplayBox)]
pub fn diplay_box() -> Html {
    let terms = use_selector(|s: &FullState| s.data.clone())
        .as_ref()
        .clone();

    let rows = terms.iter().map(row).collect_vec();

    html!(
        <table>
        <tbody>
            {rows}
        </tbody>
        </table>
    )
}

#[function_component(Categories)]
pub fn categories_dropdown() -> Html {
    let onchange = Dispatch::<FullState>::new().reduce_mut_callback_with(|s, e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        let category = PunCategory::from_str(value.as_str()).unwrap();
        s.change_category(category);
    });

    let current_category = use_selector::<FullState, _, _>(|x| x.category);

    let options = PunCategory::iter()
        .map(|category| {
            let text: &'static str = category.into();
            let selected = category == *current_category;

            html!(  <option value={text} {selected}>{text}</option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange}>
            {options}
        </select>
    )
}

pub fn row(pun: &PunPhrase) -> Html {
    html!(<tr data-tooltip={pun.phrase.full_text()}>
        <td >
        {
            pun.phrase.words.iter().enumerate()
        .map(|(i,w)| if i == pun.index{html!(<em> {pun.replacement.replacement_string.clone() + " "} </em>)} else {html!(<>{w.text.clone()+ " "} </>)})        
        .collect::<Html>()
        }

        </td>
    </tr>)
}
