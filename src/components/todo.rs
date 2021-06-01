use yew::prelude::*;

pub enum Msg {
    AddTodo,
    UpdateInput(String),
    DeleteTodo,
}

pub struct Todo {
    tasks: Vec<String>,
    input_task: String,
    link: ComponentLink<Self>,
}

impl Component for Todo {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input_task: String::from(""),
            tasks: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTodo => {
                // let new_task = String::from("new task");
                let new_task = &self.input_task;
                self.tasks.push(new_task.to_string());
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::UpdateInput(input_data) => {
                self.input_task = input_data;
                false
            }
            Msg::DeleteTodo => {
                // self.value -= 1;
                // true
                false
            }
        }
        // false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{"this is todo component"}</p>
                <div>
                <input type="text" oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value))/>
                <button onclick=self.link.callback(|_| Msg::AddTodo)> {"add"} </button>
                {for self.tasks.iter().map(|e| html!{<div>{e}</div>})}
                </div>
            </div>
        }
    }
}
