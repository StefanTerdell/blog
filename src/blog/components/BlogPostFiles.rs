use super::{
    super::server_fns::get_blog_post_file_list::get_blog_post_file_list,
    BlogPostFileListItem::BlogPostFileListItem, UploadBlogPostFileForm::UploadBlogPostFileForm,
};
use leptos::*;

#[component]
pub fn BlogPostFiles(blog_post_id: i32) -> impl IntoView {
    let file_names_resource =
        create_blocking_resource(|| (), move |_| get_blog_post_file_list(blog_post_id));

    let items = move || {
        match file_names_resource.get() {
            Some(Ok(file_names)) => {
                file_names
                    .into_iter()
                    .map(|file_name| {
                        view! { <BlogPostFileListItem file_name=file_name refetch=move || file_names_resource.refetch()/> }
                    })
                    .collect_view()
            },
            Some(Err(err)) => format!("{err:?}").into_view(),
            None => "loading".into_view(),
        }
    };

    view! {
        <div>
            <div class="divider">"Files"</div>
            <Transition>
                <table class="table w-96 mx-auto">
                    <tbody>{items()}</tbody>
                </table>
            </Transition>
            <UploadBlogPostFileForm
                blog_post_id=blog_post_id
                refetch=move || file_names_resource.refetch()
            />
        </div>
    }
}
