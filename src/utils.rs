use std::convert::Infallible;


pub fn try_or<Res, ReturnVal, E, EVal>(r_func: Res, e_func: E) -> ReturnVal
    where 
        Res: FnOnce()     -> Result<ReturnVal, EVal>,
        E:   FnOnce(EVal) -> Infallible
{
    let val = r_func();
    match val {
        Ok (v) => v,
        Err(err) => never(e_func(err))
    }
}

fn never<E>(_: E) -> ! {
    panic!()
}
