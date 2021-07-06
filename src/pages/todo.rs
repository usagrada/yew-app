use yew::prelude::*;
use crate::components::todo::Todo;

pub struct TodoPage {
	// props: Props,
	_link: ComponentLink<Self>,
}

impl Component for TodoPage {
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
			<h1>{"Todo Page"}</h1>
			<section>
				<Todo />
			</section>
			</>
		}
	}
}
