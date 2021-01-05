use crate::components::{ChatWindow, LiveStreamPlayer};

use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

pub struct LiveStream {}

impl Component for LiveStream {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="live_stream_page">
                <LiveStreamPlayer topic="livelikevideo" streamer_peer_id="12D3KooWAPZ3QZnZUJw3BgEX9F7XL383xFNiKQ5YKANiRC3NWvpo" />
                <ChatWindow />
            </div>
        }
    }
}