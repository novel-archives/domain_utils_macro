#[macro_use]
extern crate domain_utils_macro;

use domain_utils::*;

type Id<T> = domain_utils::Id<T>;
#[derive(Entity)]
struct TestStruct {
    id: Id<TestStruct>,
}

use test_case::test_case;

#[test_case("hoge","hoge"=>true)]
fn works_eq(v1: &str, v2: &str) -> bool {
    let ts1 = TestStruct {
        id: Id::new(v1.to_owned()),
    };
    let ts2 = TestStruct {
        id: Id::new(v2.to_owned()),
    };
    ts1 == ts2
}
