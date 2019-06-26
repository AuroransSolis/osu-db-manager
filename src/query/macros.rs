#[macro_export]
macro_rules! get_parse_and_assign {
    ($matches:ident { $($arg_name:literal, $var:ident => $t:ty);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {
                Some(Comparison::from_str(m)?)
            } else {
                None
            };
        )+
    }
}

#[macro_export]
macro_rules! get_and_assign_string {
    ($matches:ident { $($arg_name:literal, $var:ident);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {
                Some(Comparison::Eq(m.to_string()))
            } else {
                None
            };
        )+
    }
}

fn tmp(s: &str) {

}

#[macro_export]
macro_rules! get_and_assign_datetime {
    ($matches:ident { $($arg_name:literal, $var:ident);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {

            } else {
                None
            };
        )+
    }
}