#[macro_export]
macro_rules! gc_macros {
    // TODO: better implementation - very hacky
    ($gc:ident) => {
        macro_rules! l_input {
            () => {
                $gc.input()
            };
        }
        macro_rules! l_and {
            ($a: expr, $b: expr) => {{
                let c = $a;
                let d = $b;
                $gc.l_and(c, d)
            }};
        }
        macro_rules! l_or {
            ($a: expr, $b: expr) => {{
                let c = $a;
                let d = $b;
                $gc.l_or(c, d)
            }};
        }
        macro_rules! l_neg {
            ($a: expr) => {{
                let c = $a;
                $gc.l_neg(c)
            }};
        }
    };
}
