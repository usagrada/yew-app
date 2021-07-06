use yew::prelude::*;

pub struct AboutPage {
	// props: Props,
	_link: ComponentLink<Self>,
}

impl Component for AboutPage {
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
			<>
			<h1>{"About Page"}</h1>
			<section>
				<div>{"wgpu を使って 何かしようとして build で詰まったので，何もないページ"}</div>
			</section>
			</>
		}
	}
}
