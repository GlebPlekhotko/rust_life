use life::arguments::*;

#[test]
fn empty_arguments_string() {
    let input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,

        input_file : None,
        output_file : None
    };

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn one_argument_string() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn x_size() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 100,
        y_size : 0,

        generations : 0,
        output_each_generation : false,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-x"));
    input.push(String::from("100"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
#[should_panic]
fn x_size_illegal_value_type() {
    let mut input : Vec<String> = Vec::new();

    input.push(String::from("input"));
    input.push(String::from("-x"));
    input.push(String::from("o"));

    parse(input);
}

#[test]
fn y_size() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 100,

        generations : 0,
        output_each_generation : false,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-y"));
    input.push(String::from("100"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
#[should_panic]
fn y_size_illegal_value_type() {
    let mut input : Vec<String> = Vec::new();

    input.push(String::from("input"));
    input.push(String::from("-y"));
    input.push(String::from("o"));

    parse(input);
}

#[test]
fn generations() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 100,
        output_each_generation : false,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-g"));
    input.push(String::from("100"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
#[should_panic]
fn generations_illegal_value_type() {
    let mut input : Vec<String> = Vec::new();

    input.push(String::from("input"));
    input.push(String::from("-g"));
    input.push(String::from("o"));

    parse(input);
}

#[test]
fn output_each_generation() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : true,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-e"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn input_file() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,

        input_file : Some("input_file".to_string()),
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-i"));
    input.push(String::from("input_file"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn output_file() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,

        input_file : None,
        output_file : Some("output_file".to_string())
    };

    input.push(String::from("input"));
    input.push(String::from("-o"));
    input.push(String::from("output_file"));

    let actual = parse(input);

    assert!(expected == actual);
}
