use std::collections::HashSet;
use std::rc::Rc;

use crate::components::{Loading, Navbar, Thumbnail};
use crate::utils::{IpfsService, LocalStorage};

use wasm_bindgen_futures::spawn_local;

use yew::prelude::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::services::ConsoleService;
use yew::Callback;

use linked_data::feed::{FeedAnchor, Media};

use cid::Cid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct ContentFeed {
    props: Props,
    cb: Callback<(Cid, Result<Media>)>,

    content_set: HashSet<Cid>,
    content: Vec<(Cid, Rc<Media>)>,
}

pub enum Msg {
    Metadata((Cid, Result<Media>)),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub ipfs: IpfsService,
    pub storage: LocalStorage,
    pub feed: Rc<FeedAnchor>,
}

impl Component for ContentFeed {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut feed = Self {
            props,
            cb: link.callback(Msg::Metadata),

            content_set: HashSet::with_capacity(100),
            content: Vec::with_capacity(100),
        };

        feed.get_content();

        feed
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Metadata(result) => self.on_metadata(result),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.feed != self.props.feed {
            self.props = props;

            self.get_content();
        }

        false
    }

    fn view(&self) -> Html {
        let content = if self.content.is_empty() {
            html! {
                <Loading />
            }
        } else {
            html! {
                <>
                    {
                        for self.content.iter().rev().map(|(cid, metadata)| {
                            html! { <Thumbnail metadata_cid=*cid metadata=metadata.clone() /> }
                        })
                    }
                </>
            }
        };

        html! {
            <div class="content_feed_page">
                <Navbar />
                <div class="feed">
                    { content }
                </div>
            </div>
        }
    }
}

impl ContentFeed {
    /// IPFS dag get all metadata from content feed starting by newest.
    fn get_content(&mut self) {
        for ipld in self.props.feed.content.iter().rev() {
            if self.content_set.insert(ipld.link) {
                spawn_local({
                    let cb = self.cb.clone();
                    let ipfs = self.props.ipfs.clone();
                    let cid = ipld.link;

                    async move { cb.emit((cid, ipfs.dag_get(cid, Option::<&str>::None).await)) }
                });
            }
        }
    }

    /// Callback when IPFS dag get returns a Media node.
    fn on_metadata(&mut self, response: (Cid, Result<Media>)) -> bool {
        let (cid, metadata) = match response {
            (cid, Ok(metadata)) => (cid, metadata),
            (_, Err(e)) => {
                ConsoleService::error(&format!("{:?}", e));
                return false;
            }
        };

        #[cfg(debug_assertions)]
        ConsoleService::info("Metadata Update");

        let index = self
            .content
            .binary_search_by(|(_, probe)| probe.timestamp().cmp(&metadata.timestamp()))
            .unwrap_or_else(|x| x);

        self.content.insert(index, (cid, Rc::from(metadata)));

        true
    }
}
