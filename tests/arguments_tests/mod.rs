use life::arguments::*;

#[test]
fn empty_arguments_string() {
    let input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.0,

        fence_type : FenceType::Cliff,

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
        density : 0.0,

        fence_type : FenceType::Cliff,

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
        density : 0.0,

        fence_type : FenceType::Cliff,

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
        density : 0.0,

        fence_type : FenceType::Cliff,

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
        density : 0.0,

        fence_type : FenceType::Cliff,

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
        density : 0.0,

        fence_type : FenceType::Cliff,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-e"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn density() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.5,

        fence_type : FenceType::Cliff,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-d"));
    input.push(String::from("0.5"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
#[should_panic]
fn density_illegal_value_type() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.5,

        fence_type : FenceType::Cliff,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-d"));
    input.push(String::from("wrong"));

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
        density : 0.0,

        fence_type : FenceType::Cliff,

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
fn input_file_empty_string() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.0,

        fence_type : FenceType::Cliff,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-i"));
    input.push(String::from(""));

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
        density : 0.0,

        fence_type : FenceType::Cliff,

        input_file : None,
        output_file : Some("output_file".to_string()),
    };

    input.push(String::from("input"));
    input.push(String::from("-o"));
    input.push(String::from("output_file"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn output_file_empty_string() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.0,

        fence_type : FenceType::Cliff,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-o"));
    input.push(String::from(""));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn fence_type_cliff() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.0,

        fence_type : FenceType::Cliff,

        input_file : None,
        output_file : None,
    };

    input.push(String::from("input"));
    input.push(String::from("-f"));
    input.push(String::from("cliff"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn fence_type_fade_away() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.0,

        fence_type : FenceType::FadeAway,

        input_file : None,
        output_file : None,
    };

    input.push(String::from("input"));
    input.push(String::from("-f"));
    input.push(String::from("fade"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn fence_type_warp() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.0,

        fence_type : FenceType::Warp,

        input_file : None,
        output_file : None,
    };

    input.push(String::from("input"));
    input.push(String::from("-f"));
    input.push(String::from("warp"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn multiple_arguments() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 200,
        y_size : 100,

        generations : 10,
        output_each_generation : true,
        density : 0.5,

        fence_type : FenceType::FadeAway,

        input_file : Some("input_file".to_string()),
        output_file : Some("output_file".to_string())
    };

    input.push(String::from("input"));
    input.push(String::from("-x"));
    input.push(String::from("200"));
    input.push(String::from("-y"));
    input.push(String::from("100"));
    input.push(String::from("-g"));
    input.push(String::from("10"));
    input.push(String::from("-e"));
    input.push(String::from("-d"));
    input.push(String::from("0.5"));
    input.push(String::from("-i"));
    input.push(String::from("input_file"));
    input.push(String::from("-o"));
    input.push(String::from("output_file"));
    input.push(String::from("-f"));
    input.push(String::from("fade"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
fn multiple_arguments_reverse() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 200,
        y_size : 100,

        generations : 10,
        output_each_generation : true,
        density : 0.5,

        fence_type : FenceType::FadeAway,

        input_file : Some("input_file".to_string()),
        output_file : Some("output_file".to_string())
    };

    input.push(String::from("input"));
    input.push(String::from("-f"));
    input.push(String::from("fade"));
    input.push(String::from("-o"));
    input.push(String::from("output_file"));
    input.push(String::from("-i"));
    input.push(String::from("input_file"));
    input.push(String::from("-d"));
    input.push(String::from("0.5"));
    input.push(String::from("-e"));
    input.push(String::from("-g"));
    input.push(String::from("10"));
    input.push(String::from("-y"));
    input.push(String::from("100"));
    input.push(String::from("-x"));
    input.push(String::from("200"));

    let actual = parse(input);

    assert!(expected == actual);
}

#[test]
#[should_panic]
fn no_arguments_value() {
    let mut input : Vec<String> = Vec::new();
    let expected = Arguments {
        x_size : 0,
        y_size : 0,

        generations : 0,
        output_each_generation : false,
        density : 0.0,

        fence_type : FenceType::Cliff,

        input_file : None,
        output_file : None
    };

    input.push(String::from("input"));
    input.push(String::from("-x"));

    let actual = parse(input);

    assert!(expected == actual);
}
