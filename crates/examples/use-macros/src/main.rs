use rs_flow::prelude::*;


#[inputs {
    message => { description = "Description from input 'message'" },
    data,
    bar => {},
}]
#[outputs {
    foo => { description = "Output description"}
}]
struct UseMacro {
    name: String
}

fn main() {
    let use_macro_component = UseMacro { name: "my string".to_owned() };

    println!("{}", use_macro_component.name);
    println!("{:#?}", use_macro_component.inputs());
    println!("{:#?}", use_macro_component.outputs());

    assert_eq!(0, use_macro_component.input("message"));
    assert_eq!(1, use_macro_component.input("data"));
    assert_eq!(2, use_macro_component.input("bar"));

    //use_macro_component.input("invalid"); //panic

    //use_macro_component.output("invalid"); //panic
    assert_eq!(0, use_macro_component.output("foo"));
}