#[allow(unused_imports)]
use crate::{common::*, engine::*, error::*, parse::*};

macro_rules! gen_test {
    ($name:ident, should_err = $should_err:expr, $err_pat:pat = err) => {
        mod $name {
            use super::*;

            #[test]
            pub fn test() -> Result<(), crate::Error> {
                let name_str = stringify!($name);
                let in_filename = format!("test_cases/{}_input.csv", name_str);
                let out_filename = format!("test_cases/{}_output.csv", name_str);

                let mut engine = PaymentsEngine::new();
                let res = engine.run_from_file(&in_filename);

                if $should_err {
                    assert!(res.is_err());

                    let err = res.unwrap_err();
                    println!("error: {:?}", err);
                    assert!(matches!(err, $err_pat));
                    return Ok(());
                }

                assert!(res.is_ok());

                let expected_out = std::fs::read_to_string(out_filename)
                    .map_err(|e| Error::Other(format!("IO Error: {}", e)))?;

                let engine_out = engine.to_csv_string_sorted();

                assert_eq!(expected_out, engine_out);

                Ok(())
            }
        }
    };

    ($name:ident, $err_pat:pat = err) => {
        gen_test!($name, should_err = true, $err_pat = err);
    };

    ($name:ident) => {
        gen_test!($name, should_err = false, _ = err);
    };
}

macro_rules! gen_tests {
    {$($name:ident $(, $err_pat:pat = err)?);*$(;)?} => {
        $(
            gen_test! { $name $(, $err_pat = err)? }
        )*
    };
}

gen_tests! {
    t0;
    t1;
    t2;
    t3;
    t4;
    t5, Error::Parse(ParseError(0, PaymentCommandParseError::MissingHeader(_))) = err;
    t6, Error::Runtime(RuntimeError(7, EngineError::ClientIdMismatch(2, 1))) = err;
}
