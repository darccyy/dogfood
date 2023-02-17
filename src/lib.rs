mod info;

use leptos::*;

use crate::info::{get_info, Categories, Entry, Rows};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let cats = get_info();

    view! { cx,
        <Table info={cats} />
    }
}

#[component]
fn Table(cx: Scope, info: Categories) -> impl IntoView {
    let rows = info.rows();

    view! { cx,
        <table>

            <tr>
                <th> "Yes" </th>
                <th> "Maybe" </th>
                <th> "No" </th>
            </tr>

            { rows.iter().map(|row|
                Row(cx, row)
            ).collect::<Vec<_>>() }

        </table>
    }
}

#[allow(non_snake_case)]
fn Row<'a>(cx: Scope, row: &'a [Option<&'a Entry>]) -> impl IntoView {
    view! { cx,
        <tr>
            { row.iter().map(|entry|
                Cell(cx, entry)
            ).collect::<Vec<_>>() }
        </tr>
    }
}

#[allow(non_snake_case)]
fn Cell<'a>(cx: Scope, entry: &'a Option<&'a Entry>) -> impl IntoView {
    let Some(entry) = entry else {
        return view! { cx,
            <td> </td>
        };
    };

    view! { cx,
        <td>
            {&entry.name}
        </td>
    }
}
