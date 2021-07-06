use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::MyRoute;

pub struct Header {
	// props: Props,
	_link: ComponentLink<Self>,
}

impl Component for Header {
	type Message = ();
	type Properties = ();

	fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
		Self { _link }
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		html! {
			<header>
				<Link<MyRoute> classes=classes!("navbar-item") route=MyRoute::Home>
					{ "Home" }
				</Link<MyRoute>>
				<Link<MyRoute> classes=classes!("navbar-item") route=MyRoute::Todo>
					{ "Todo" }
				</Link<MyRoute>>
				<Link<MyRoute> classes=classes!("navbar-item") route=MyRoute::WebGpu>
					{ "WebGPU?" }
				</Link<MyRoute>>
			</header>
		}
	}
}
