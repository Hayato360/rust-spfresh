use leptos::*;
use leptos::ev::SubmitEvent;

use crate::api;
use crate::models::{InsertReviewRequest, SearchResponse};

#[derive(Clone, Debug)]
enum MessageType {
    Success,
    Error,
}

#[component]
pub fn HomePage() -> impl IntoView {
    let (stats, set_stats) = create_signal::<Option<(usize, usize)>>(None);
    let (loading, set_loading) = create_signal(true);

    create_effect(move |_| {
        spawn_local(async move {
            match api::get_stats().await {
                Ok(data) => {
                    set_stats.set(Some(data));
                    set_loading.set(false);
                }
                Err(e) => {
                    log::error!("Failed to fetch stats: {}", e);
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <div class="px-4 py-6">
            <div class="bg-white shadow-sm rounded-lg p-6">
                <h1 class="text-3xl font-bold text-gray-900 mb-6">
                    "Review Semantic Search Platform"
                </h1>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                    <div class="bg-blue-50 p-6 rounded-lg">
                        <h2 class="text-xl font-semibold text-blue-900 mb-2">
                            "Add Reviews"
                        </h2>
                        <p class="text-blue-700 mb-4">
                            "Insert new product reviews into the semantic search index."
                        </p>
                        <a href="/insert" class="inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
                            "Add Review →"
                        </a>
                    </div>
                    
                    <div class="bg-green-50 p-6 rounded-lg">
                        <h2 class="text-xl font-semibold text-green-900 mb-2">
                            "Search Reviews"
                        </h2>
                        <p class="text-green-700 mb-4">
                            "Find similar reviews using semantic search powered by AI embeddings."
                        </p>
                        <a href="/search" class="inline-flex items-center px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700">
                            "Search →"
                        </a>
                    </div>
                </div>

                <div class="border-t pt-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">
                        "System Statistics"
                    </h3>
                    {move || {
                        if loading.get() {
                            view! {
                                <div class="text-gray-500">
                                    "Loading statistics..."
                                </div>
                            }.into_view()
                        } else {
                            match stats.get() {
                                Some((reviews_count, vectors_count)) => view! {
                                    <div class="grid grid-cols-2 gap-4">
                                        <div class="bg-gray-50 p-4 rounded">
                                            <div class="text-2xl font-bold text-gray-900">{reviews_count}</div>
                                            <div class="text-sm text-gray-500">"Total Reviews"</div>
                                        </div>
                                        <div class="bg-gray-50 p-4 rounded">
                                            <div class="text-2xl font-bold text-gray-900">{vectors_count}</div>
                                            <div class="text-sm text-gray-500">"Vector Embeddings"</div>
                                        </div>
                                    </div>
                                }.into_view(),
                                None => view! {
                                    <div class="text-red-500">
                                        "Failed to load statistics"
                                    </div>
                                }.into_view()
                            }
                        }
                    }}
                </div>

                <div class="border-t mt-6 pt-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">
                        "How it works"
                    </h3>
                    <div class="text-gray-600 space-y-2">
                        <p>"• File-based storage - no database required"</p>
                        <p>"• Simple TF-IDF style embeddings for semantic matching"</p>
                        <p>"• Vector similarity search for finding related reviews"</p>
                        <p>"• Append-only data structure for persistent storage"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn InsertPage() -> impl IntoView {
    let (review_title, set_review_title) = create_signal(String::new());
    let (review_body, set_review_body) = create_signal(String::new());
    let (product_id, set_product_id) = create_signal(String::new());
    let (rating, set_rating) = create_signal(5u8);
    let (submitting, set_submitting) = create_signal(false);
    let (message, set_message) = create_signal::<Option<String>>(None);
    let (message_type, set_message_type) = create_signal::<MessageType>(MessageType::Success);

    let submit_review = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        if review_title.get().trim().is_empty() || review_body.get().trim().is_empty() || product_id.get().trim().is_empty() {
            set_message.set(Some("Please fill in all fields".to_string()));
            set_message_type.set(MessageType::Error);
            return;
        }

        set_submitting.set(true);
        set_message.set(None);

        let request = InsertReviewRequest {
            review_title: review_title.get(),
            review_body: review_body.get(),
            product_id: product_id.get(),
            review_rating: rating.get(),
        };

        spawn_local(async move {
            match api::insert_review(request).await {
                Ok(_) => {
                    set_message.set(Some("Review added successfully!".to_string()));
                    set_message_type.set(MessageType::Success);
                    
                    // Clear form
                    set_review_title.set(String::new());
                    set_review_body.set(String::new());
                    set_product_id.set(String::new());
                    set_rating.set(5);
                }
                Err(e) => {
                    set_message.set(Some(format!("Failed to add review: {}", e)));
                    set_message_type.set(MessageType::Error);
                }
            }
            set_submitting.set(false);
        });
    };

    view! {
        <div class="px-4 py-6">
            <div class="bg-white shadow-sm rounded-lg p-6 max-w-2xl mx-auto">
                <h1 class="text-2xl font-bold text-gray-900 mb-6">
                    "Add New Review"
                </h1>

                {move || {
                    message.get().map(|msg| {
                        let (bg_color, text_color) = match message_type.get() {
                            MessageType::Success => ("bg-green-50", "text-green-700"),
                            MessageType::Error => ("bg-red-50", "text-red-700"),
                        };
                        view! {
                            <div class=format!("{} {} p-4 rounded-md mb-6", bg_color, text_color)>
                                {msg}
                            </div>
                        }.into_view()
                    })
                }}

                <form on:submit=submit_review class="space-y-6">
                    <div>
                        <label for="product_id" class="block text-sm font-medium text-gray-700 mb-2">
                            "Product ID"
                        </label>
                        <input
                            type="text"
                            id="product_id"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="e.g., PROD-123"
                            prop:value=move || product_id.get()
                            on:input=move |e| set_product_id.set(event_target_value(&e))
                            required
                        />
                    </div>

                    <div>
                        <label for="review_title" class="block text-sm font-medium text-gray-700 mb-2">
                            "Review Title"
                        </label>
                        <input
                            type="text"
                            id="review_title"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="e.g., Great product!"
                            prop:value=move || review_title.get()
                            on:input=move |e| set_review_title.set(event_target_value(&e))
                            required
                        />
                    </div>

                    <div>
                        <label for="review_body" class="block text-sm font-medium text-gray-700 mb-2">
                            "Review Content"
                        </label>
                        <textarea
                            id="review_body"
                            rows="6"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Write your detailed review here..."
                            prop:value=move || review_body.get()
                            on:input=move |e| set_review_body.set(event_target_value(&e))
                            required
                        ></textarea>
                    </div>

                    <div>
                        <label for="rating" class="block text-sm font-medium text-gray-700 mb-2">
                            "Rating"
                        </label>
                        <select
                            id="rating"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            prop:value=move || rating.get().to_string()
                            on:change=move |e| {
                                if let Ok(val) = event_target_value(&e).parse::<u8>() {
                                    set_rating.set(val);
                                }
                            }
                        >
                            <option value="1">"1 Star"</option>
                            <option value="2">"2 Stars"</option>
                            <option value="3">"3 Stars"</option>
                            <option value="4">"4 Stars"</option>
                            <option value="5" selected>"5 Stars"</option>
                        </select>
                    </div>

                    <button
                        type="submit"
                        class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
                        disabled=move || submitting.get()
                    >
                        {move || if submitting.get() { "Adding Review..." } else { "Add Review" }}
                    </button>
                </form>
            </div>
        </div>
    }
}

#[component]
pub fn SearchPage() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (results, set_results) = create_signal::<Option<SearchResponse>>(None);
    let (searching, set_searching) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);

    let search_reviews = move |_| {
        if query.get().trim().is_empty() {
            set_error.set(Some("Please enter a search query".to_string()));
            return;
        }

        set_searching.set(true);
        set_error.set(None);
        set_results.set(None);

        let search_query = query.get();
        spawn_local(async move {
            match api::search_reviews(search_query, Some(10)).await {
                Ok(response) => {
                    set_results.set(Some(response));
                }
                Err(e) => {
                    set_error.set(Some(format!("Search failed: {}", e)));
                }
            }
            set_searching.set(false);
        });
    };

    view! {
        <div class="px-4 py-6">
            <div class="bg-white shadow-sm rounded-lg p-6">
                <h1 class="text-2xl font-bold text-gray-900 mb-6">
                    "Semantic Review Search"
                </h1>

                <div class="mb-6">
                    <div class="flex gap-4">
                        <input
                            type="text"
                            class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Search for reviews... (e.g., 'great battery life', 'poor quality')"
                            prop:value=move || query.get()
                            on:input=move |e| set_query.set(event_target_value(&e))
                        />
                        <button
                            on:click=move |_| search_reviews(())
                            class="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50"
                            disabled=move || searching.get()
                        >
                            {move || if searching.get() { "Searching..." } else { "Search" }}
                        </button>
                    </div>
                </div>

                {move || {
                    error.get().map(|err| view! {
                        <div class="bg-red-50 text-red-700 p-4 rounded-md mb-6">
                            {err}
                        </div>
                    }.into_view())
                }}

                {move || {
                    results.get().map(|response| {
                        if response.reviews.is_empty() {
                            view! {
                                <div class="text-gray-500 text-center py-8">
                                    "No reviews found for your search query."
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div>
                                    <div class="mb-4 text-sm text-gray-600">
                                        "Found " {response.total_found} " matching reviews (showing top " {response.reviews.len()} ")"
                                    </div>
                                    <div class="space-y-4">
                                        {response.reviews.into_iter().map(|item| {
                                            let review = item.review;
                                            let score = item.similarity_score;
                                            view! {
                                                <div class="border rounded-lg p-4">
                                                    <div class="flex justify-between items-start mb-2">
                                                        <h3 class="font-semibold text-lg">{review.review_title}</h3>
                                                        <div class="text-sm">
                                                            <span class="bg-blue-100 text-blue-800 px-2 py-1 rounded">
                                                                "Similarity: " {format!("{:.1}%", score * 100.0)}
                                                            </span>
                                                        </div>
                                                    </div>
                                                    <p class="text-gray-700 mb-3">{review.review_body}</p>
                                                    <div class="flex justify-between text-sm text-gray-500">
                                                        <span>"Product: " {review.product_id}</span>
                                                        <span>"Rating: " {review.review_rating} "/5"</span>
                                                        <span>{review.timestamp}</span>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }.into_view()
                        }
                    })
                }}
            </div>
        </div>
    }
}
