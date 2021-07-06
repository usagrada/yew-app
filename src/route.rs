// use crate::layout::Layout;
extern crate yewapp;
use yewapp::pages::{webgpu::WebGPUPage, todo::TodoPage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum MyRoute {
	#[at("/")]
	Home,
    #[at("/web-gpu")]
    WebGpu,
	#[at("/todo")]
 	Todo,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub struct Route {
	_link: ComponentLink<Self>,
}
impl Component for Route {
	type Message = ();
	type Properties = ();

	fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
		Self { _link }
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			// <Layout>
				<Router<MyRoute> render=Router::render(switch) />
			// </Layout>
		}
	}
}

fn switch(routes: &MyRoute) -> Html {
	match routes {
		MyRoute::Home => {
			html! {<div>{"home"}</div>}
		}
        MyRoute::WebGpu => {
            html! {
                <WebGPUPage/>
            }
        }
		MyRoute::Todo => {
			html! {
				<TodoPage />
			}
		}
		_ => {
			html! {<div>{"404 Not Found"}</div>}
		}
	}
}