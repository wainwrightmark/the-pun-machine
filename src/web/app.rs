use std::{convert::*, str::*};

use itertools::Itertools;
use strum::IntoEnumIterator;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{core::prelude::*, state::prelude::*};

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
        .unwrap_or_default();

    if err.is_empty() {
        html!(<> </>)
    } else {
        html!(<code> {err} </code>)
    }
}

#[function_component(DisplayBox)]
pub fn diplay_box() -> Html {
    let data = use_selector(|s: &FullState| s.data.clone());

    let terms = data
        .iter()
        .sorted_by_key(|x| x.replacement.pun_word.clone())
        .group_by(|x| x.replacement.pun_word.clone())
        .into_iter()
        .map(|x| (x.0, x.1.cloned().collect_vec()))
        .sorted_by_key(|x| -(x.1.len() as isize))
        .collect_vec();

    //.collect_vec();

    let rows = terms
        .iter()
        .map(|(row_key,phrases)| html!(<RowGroup key={row_key.clone()} row_key={row_key.clone()} phrases={phrases.clone()}/>))
        .collect_vec();

    html!(
        <table>
        {rows}
        </table>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct RowGroupProperties {
    pub row_key: &'static str,
    pub phrases: Vec<PunPhrase>,
}

#[function_component(RowGroup)]
pub fn row_group(properties: &RowGroupProperties) -> Html {
    let show_category = *use_selector(|s: &FullState| s.category.is_none()).as_ref();

    let row_key = properties.row_key;

    let hidden = !*use_selector(|s: &FullState| s.visible_groups.contains(row_key)).as_ref();

    if properties.phrases.len() == 1 {
        html!(
            <tbody>
                {row(&properties.phrases[0], show_category)}
            </tbody>
        )
    } else {
        let label = format!("{} ({})", properties.row_key, properties.phrases.len());
        let rows = properties
            .phrases
            .iter()
            .map(|x| row(x, show_category))
            .collect_vec();

        let colspan = if show_category { "3" } else { "2" };

        let onclick = Dispatch::<FullState>::new().reduce_mut_callback_with(move |s, _| {
            s.toggle_group_visibility(&row_key);
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
