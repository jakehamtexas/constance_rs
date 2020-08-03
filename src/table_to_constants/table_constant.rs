pub struct SimpleEnum {}
pub struct SimpleEnumWithDescription {}
pub struct StringEnum {}
pub struct StringEnumWithDescription {}
pub struct ObjectLike {}
pub struct ObjectLikeWithDescription {}
pub enum TableConstant {
    SimpleEnum(SimpleEnum),
    SimpleEnumWithDescription(SimpleEnumWithDescription),
    StringEnum(StringEnum),
    StringEnumWithDescription(StringEnumWithDescription),
    ObjectLike(ObjectLike),
    ObjectLikeWithDescription(ObjectLikeWithDescription),
}
