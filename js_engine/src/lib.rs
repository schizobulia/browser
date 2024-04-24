use rusty_v8 as v8;

pub fn run_js(code: &str) {

    // Initialize V8.
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    run_script(code);

    unsafe {
        v8::V8::dispose();
    }
    v8::V8::shutdown_platform();
}

fn run_script(source_code: &str) {
    // Create a new Isolate and make it the current one.
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

    // Create a stack-allocated handle scope.
    let handle_scope: &mut v8::HandleScope<'_, ()> = &mut v8::HandleScope::new(isolate);

    // Create a new global object template and add a function to it.
    let global = v8::ObjectTemplate::new(handle_scope);
    // global.set(
    //     v8::String::new(handle_scope, "rustFunction")
    //         .unwrap()
    //         .into(),
    //     v8::FunctionTemplate::new(handle_scope, rust_function).into(),
    // );

    let document = v8::ObjectTemplate::new(handle_scope);
    document.set(
        v8::String::new(handle_scope, "getElementById")
            .unwrap()
            .into(),
        v8::FunctionTemplate::new(handle_scope, get_element_by_id).into(),
    );
    global.set(
        v8::String::new(handle_scope, "document").unwrap().into(),
        document.into(),
    );

    let context = v8::Context::new_from_template(handle_scope, global);

    // Enter the context for compiling and running the hello world script.
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let code = v8::String::new(scope, source_code).unwrap();

    // Compile the source code.
    let script = v8::Script::compile(scope, code, None).unwrap();

    // Run the script to get the result.
    let result = script.run(scope).unwrap();

    // Convert the result to a string and print it.
    let result = result.to_string(scope).unwrap();
    println!("{}", result.to_rust_string_lossy(scope));
}

fn get_element_by_id(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let arg1 = args.get(0);
    let arg1 = arg1.to_string(scope).unwrap();
    println!(
        "Rust function called with argument: {}",
        arg1.to_rust_string_lossy(scope)
    );
}
