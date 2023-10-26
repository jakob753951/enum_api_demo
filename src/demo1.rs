pub fn main() {
	let state: State<String> = State {
		data: None,
		loading: true,
		error: None,
	};
	let result = get_component(&state);
	println!("Using {state:?}:\n'{result}'");
	
	let state: State<String> = State {
		data: Some("Here's the data".to_string()),
		loading: false,
		error: None,
	};
	let result = get_component(&state);
	println!("Using {state:?}:\n'{result}'");
	
	let state: State<String> = State {
		data: None,
		loading: false,
		error: Some("Couldn't get data".to_string()),
	};
	let result = get_component(&state);
	println!("Using {state:?}:\n'{result}'");
	
	let state: State<String> = State {
		data: Some("There's data here".to_string()),
		loading: true,
		error: Some("But we're loading and we have an error".to_string()),
	};
	let result = get_component(&state);
	println!("Using {state:?}:\n'{result}'");
	
	let state: State<String> = State {
		data: None,
		loading: false,
		error: None,
	};
	let result = get_component(&state);
	println!("Using {state:?}:\n'{result}'");
}

fn get_component(state: &State<String>) -> String {
	if state.loading {
		return "Loading...".to_string();
	}
	
	if state.error.is_some() {
		return "Error".to_string();
	}
	
	match &state.data {
		Some(data) => data.clone(),
		None => "Shouldn't be able to get to here".to_string()
	}
}

#[derive(Debug)]
struct State<D> {
	data: Option<D>,
	loading: bool,
	error: Option<String>,
}