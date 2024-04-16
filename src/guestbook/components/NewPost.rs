use super::super::server_fns::create_guestbook_post::CreateGuestbookPost;
use crate::github::components::LogOutButton;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn NewPost<F: Fn() + 'static>(refetch_posts: F) -> impl IntoView {
    let create_post = create_server_action::<CreateGuestbookPost>();

    create_effect(move |_| {
        if create_post.value().get().is_some() {
            refetch_posts();
        };
    });

    view! {
        <>
            {move || match create_post.value().get() {
                Some(Ok(_)) => {
                    view! { <i>"Thanks for posting in my guestbook! 💖"</i> }.into_view()
                }
                Some(Err(err)) => format!("{err:?}").into_view(),
                None => {
                    view! {
                        <div class="flex mx-auto">
                            <ActionForm action=create_post>
                                <div class="join">
                                    <input
                                        placeholder="Say something nice :)"
                                        class="input input-bordered input-sm mr-2 join-item"
                                        disabled=move || create_post.pending().get()
                                        type="text"
                                        min=3
                                        name="content"
                                    />
                                    <input
                                        class="btn btn-sm mr-2 join-item"
                                        disabled=move || create_post.pending().get()
                                        type="submit"
                                        value="Submit"
                                    />
                                </div>

                            </ActionForm>
                            <LogOutButton small=true neutral=true/>
                        </div>
                    }
                        .into_view()
                }
            }}
        </>
    }
}
