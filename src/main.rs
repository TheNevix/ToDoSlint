use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError>{
    let main = MainApp::new()?;

    let todos: Vec<String> = vec![
        "Todo 1".to_string(),
        "Todo 2".to_string(),
        "Todo 3".to_string(),
        "Todo 4".to_string(),
        "Todo 5".to_string(),
        "Todo 6".to_string(),
        "Todo 7".to_string(),
        "Todo 8".to_string(),
        "Todo 9".to_string(),
        "Todo 10".to_string(),
        "Todo 11".to_string(),
        "Todo 12".to_string(),
        "Todo 13".to_string(),
        "Todo 14".to_string(),
        "Todo 15".to_string(),
        "Todo 16".to_string(),
        "Todo 17".to_string(),
        "Todo 18".to_string(),

    ];

    // Step 2: Convert Vec<String> to Vec<slint::SharedString>
    let my_vec_shared: Vec<SharedString> = todos.into_iter().map(Into::into).collect();

    // Step 3: Create a VecModel from the Vec<slint::SharedString>
    let model = ModelRc::new(VecModel::from(my_vec_shared));

    main.set_todos(model);


    main.run()
}
