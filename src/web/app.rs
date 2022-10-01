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

#[function_component(Categories)]
pub fn categories_dropdown() -> Html {
    let onchange = Dispatch::<FullState>::new().reduce_mut_callback_with(|s, e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();

        let category = Category::from_str(value.as_str()).ok();
        s.change_category(category);
    });

    let current_category = use_selector::<FullState, _, _>(|x| x.category);

    let options = Category::iter()
        .map(|category| {
            let text: &'static str = category.into();
            let selected = Some(category) == *current_category;

            html!(  <option value={text} {selected}>{category}</option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange}>
        <option value={""} selected={current_category.is_none()}>{"Any"}</option>
            {options}
        </select>
    )
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

    if err.is_empty() {
        html!(<> </>)
    } else {
        html!(<code> {err} </code>)
    }
}

#[function_component(DisplayBox)]
pub fn diplay_box() -> Html {
    let terms = use_selector(|s: &FullState| {
        s.data
            .iter()
            .map(|x| &x.replacement.pun_word)
            .unique()
            .cloned()
            .collect_vec()
    });

    //.collect_vec();

    let rows = terms
        .iter()
        .map(|key| html!(<RowGroup row_key={key.clone()}/>))
        .collect_vec();

    html!(
        <table>
        {rows}
        </table>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct RowGroupProperties {
    pub row_key: String,
}

#[function_component(RowGroup)]
pub fn row_group(properties: &RowGroupProperties) -> Html {
    let key = properties.row_key.clone();
    let show_category = use_selector(|s: &FullState| s.category.is_none())
        .as_ref()
        .clone();
    let phrases = use_selector(move |s: &FullState| {
        s.data
            .iter()
            .filter(|x| x.replacement.pun_word == key)
            .cloned()
            .collect_vec()
    });

    let key2 = properties.row_key.clone();
    let hidden = !use_selector(move |s: &FullState| s.visible_groups.contains(&key2))
        .as_ref()
        .clone();

    if phrases.len() == 1 {
        html!(
            <tbody>
                {row(&phrases[0], show_category)}
            </tbody>
        )
    } else {
        let label = format!("{} ({})", properties.row_key, phrases.len());
        let rows = phrases.iter().map(|x| row(x, show_category)).collect_vec();
        let key3 = properties.row_key.clone();

        let colspan = if show_category{"3"} else{"2"};

        let onclick = Dispatch::<FullState>::new().reduce_mut_callback_with(move |s, _| {
            s.toggle_group_visibility(&key3);
        });

        html!(
            <>
            <tbody>
                <tr>
                <td colspan={colspan}>
                <button {onclick}>{label}</button>
                </td>

                </tr>
            </tbody>
            <tbody hidden={hidden}>
                {rows}
            </tbody>
            </>

        )
    }
}

pub fn row(pun: &PunPhrase, show_category: bool) -> Html {
    html!(<tr data-tooltip={pun.phrase.text.clone()}>
        <td >
        {
            pun.phrase.words.iter().enumerate()
        .map(|(i,w)| if i == pun.index{html!(<em> {pun.replacement.replacement_string.clone() + " "} </em>)} else {html!(<>{w.text.clone()+ " "} </>)})        
        .collect::<Html>()
        }

        </td>
        // <td>
        // {pun.replacement.pun_type}
        // </td>
        if show_category{
            <td>
        {pun.phrase.category}
        </td>
        }

    </tr>)
}
