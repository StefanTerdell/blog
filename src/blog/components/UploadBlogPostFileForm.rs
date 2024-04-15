use super::super::server_fns::upload_blog_post_file::upload_blog_post_file;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, FormData, HtmlFormElement, HtmlInputElement, SubmitEvent};

#[component]
pub fn UploadBlogPostFileForm<F: Fn() + 'static>(blog_post_id: i32, refetch: F) -> impl IntoView {
    let (file_name, set_file_name) = create_signal("AAAAA".to_string());

    let handle_file_name_change = move |ev: Event| {
        let mut file_name = ev
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        if file_name.starts_with("C:\\fakepath\\") {
            file_name = file_name[12..].to_string();
        }

        set_file_name(file_name);
    };

    let upload_action =
        create_action(move |data: &FormData| upload_blog_post_file(data.clone().into()));

    let handle_submit = move |ev: SubmitEvent| {
        ev.stop_propagation();
        ev.prevent_default();
        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let form_data = FormData::new_with_form(&target).unwrap();

        upload_action.dispatch(form_data);
    };

    create_effect(move |_| {
        if matches!(upload_action.value().get(), Some(Ok(_))) {
            refetch();
        }
    });

    view! {
        <form on:submit=handle_submit>
            <input type="hidden" name="blog_post_id" value=blog_post_id/>
            <input type="hidden" name="file_name" prop:value=file_name/>
            <div class="join">
                <input
                    type="file"
                    class="file-input file-input-bordered file-input-sm join-item"
                    name="file_to_upload"
                    on:change=handle_file_name_change
                />
                <input type="submit" class="btn btn-sm join-item" value="upload"/>
            </div>
        </form>
        {move || {
            upload_action
                .value()
                .get()
                .map(|v| match v {
                    Ok(bytes) => view! { <p>{format!("Uploaded {bytes} bytes")}</p> },
                    Err(err) => view! { <p>{format!("{err:?}")}</p> },
                })
        }}
    }
}
