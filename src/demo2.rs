pub fn main() {
	let state: State<String> = State::Loading;
	let result = get_component(&state);
	println!("Using state {state:?}:\n'{result}'");
	
	let state: State<String> = State::Done("Here's the data".to_string());
	let result = get_component(&state);
	println!("Using state {state:?}:\n'{result}'");
	
	let state: State<String> = State::Error("Couldn't get data".to_string());
	let result = get_component(&state);
	println!("Using state {state:?}:\n'{result}'");
	
	// We can't have both
	// let state: State<String> = State {
	// 	data: Some("There's data here".to_string()),
	// 	loading: true,
	// 	error: Some("But we're loading and we have an error".to_string()),
	// };
	// let result = get_component(state);
	// println!("{result}");
}

fn get_component(state: &State<String>) -> String {
	match state {
		State::Done(data) => data.clone(),
		State::Loading => "loading...".to_string(),
		State::Error(e) => e.to_string(),
	}
}

#[derive(Debug)]
enum State<D> {
	Done(D),
	Loading,
	Error(String),
}