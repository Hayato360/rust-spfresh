use leptos::*;
use leptos_router::*;

use crate::pages::{HomePage, InsertPage, SearchPage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="min-h-screen bg-gray-50">
                <nav class="bg-white shadow-sm border-b">
                    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        <div class="flex justify-between h-16">
                            <div class="flex space-x-8">
                                <A href="/" class="flex items-center px-3 py-2 text-sm font-medium text-gray-900 hover:text-blue-600">
                                    "Home"
                                </A>
                                <A href="/insert" class="flex items-center px-3 py-2 text-sm font-medium text-gray-500 hover:text-blue-600">
                                    "Add Review"
                                </A>
                                <A href="/search" class="flex items-center px-3 py-2 text-sm font-medium text-gray-500 hover:text-blue-600">
                                    "Search"
                                </A>
                            </div>
                        </div>
                    </div>
                </nav>
                
                <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/insert" view=InsertPage/>
                        <Route path="/search" view=SearchPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
