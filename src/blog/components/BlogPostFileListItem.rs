use super::super::server_fns::delete_blog_post_file::DeleteBlogPostFile;
use leptos::*;

#[component]
pub fn BlogPostFileListItem<F: Fn() + 'static>(file_name: String, refetch: F) -> impl IntoView {
    let action = create_server_action::<DeleteBlogPostFile>();
    let ffs = file_name.clone();
    let handle_click = move |_| {
        action.dispatch(DeleteBlogPostFile {
            file_name: file_name.clone(),
        });
    };

    create_effect(move |_| {
        if matches!(action.value().get(), Some(_)) {
            refetch();
        }
    });

    view! {
        <tr>
            <td>
                <a class="link" href=format!("/blog-files/{ffs}") target="_blank">
                    {ffs}
                </a>
            </td>
            <td class="text-right">
                <button on:click=handle_click>"Delete"</button>
            </td>
        </tr>
    }
}
