#[doc(hidden)]
pub use paste::paste;

#[macro_export]
macro_rules! run_fn_on_instance {
    ($fn_name:ident, $path_to_instance:expr) => {
        $fn_name($path_to_instance)
    };
}

#[rust_analyzer::skip]
#[macro_export]
macro_rules! test_fn_on_all_instances {
    ($fn_name:ident, $name:ident) => {
        $crate::paste! {
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _a280>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/a280.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _ali535>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/ali535.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _att48>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/att48.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _att532>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/att532.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _berlin52>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/berlin52.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _bier127>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/bier127.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _brd14051>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/brd14051.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _burma14>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/burma14.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _ch130>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/ch130.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _ch150>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/ch150.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _d1291>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/d1291.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _d15112>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/d15112.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _d1655>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/d1655.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _d18512>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/d18512.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _d198>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/d198.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _d2103>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/d2103.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _d493>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/d493.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _d657>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/d657.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _dsj1000>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/dsj1000.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _eil101>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/eil101.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _eil51>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/eil51.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _eil76>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/eil76.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _fl1400>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/fl1400.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _fl1577>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/fl1577.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _fl3795>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/fl3795.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _fl417>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/fl417.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _fnl4461>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/fnl4461.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _gil262>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/gil262.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _gr137>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/gr137.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _gr202>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/gr202.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _gr229>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/gr229.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _gr431>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/gr431.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _gr666>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/gr666.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _gr96>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/gr96.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroA100>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroA100.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroA150>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroA150.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroA200>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroA200.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroB100>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroB100.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroB150>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroB150.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroB200>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroB200.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroC100>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroC100.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroD100>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroD100.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _kroE100>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/kroE100.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _lin105>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/lin105.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _lin318>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/lin318.tsp");
            }
            // Fixed Edges
            // TODO: Re-enable when implemented
            // #[test]
            // #[allow(non_snake_case)]
            // fn [<$name _linhp318>]() {
            //     $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/linhp318.tsp");
            // }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _nrw1379>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/nrw1379.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _p654>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/p654.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pcb1173>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pcb1173.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pcb3038>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pcb3038.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pcb442>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pcb442.tsp");
            }
            // Just too big
            // #[test]
            // #[allow(non_snake_case)]
            // fn [<$name _pla33810>]() {
            //     $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pla33810.tsp");
            // }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pla7397>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pla7397.tsp");
            }
            // Just too big
            // #[test]
            // #[allow(non_snake_case)]
            // fn [<$name _pla85900>]() {
            //     $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pla85900.tsp");
            // }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr1002>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr1002.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr107>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr107.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr124>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr124.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr136>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr136.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr144>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr144.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr152>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr152.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr226>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr226.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr2392>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr2392.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr264>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr264.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr299>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr299.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr439>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr439.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _pr76>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/pr76.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rat195>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rat195.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rat575>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rat575.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rat783>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rat783.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rat99>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rat99.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rd100>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rd100.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rd400>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rd400.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rl11849>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rl11849.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rl1304>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rl1304.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rl1323>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rl1323.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rl1889>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rl1889.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rl5915>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rl5915.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _rl5934>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/rl5934.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _st70>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/st70.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _ts225>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/ts225.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _tsp225>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/tsp225.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _u1060>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/u1060.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _u1432>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/u1432.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _u159>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/u159.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _u1817>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/u1817.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _u2152>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/u2152.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _u2319>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/u2319.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _u574>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/u574.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _u724>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/u724.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _ulysses16>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/ulysses16.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _ulysses22>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/ulysses22.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _usa13509>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/usa13509.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _vm1084>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/vm1084.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _vm1748>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsplib_symmetric/vm1748.tsp");
            }
            #[test]
            #[allow(non_snake_case)]
            fn [<$name _12>]() {
                $crate::run_fn_on_instance!($fn_name, "../../instances/tsp_rust/12.tsp");
            }
        }
    };
}
