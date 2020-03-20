use id_derive as id;

#[derive(Debug, PartialEq, Eq, id::Id)]
struct Id(usize);

#[test]
fn test_value_is_valid() {
    assert_eq!(Id(12).0, 12);
}

#[test]
fn test_print() {
    #[derive(id::Display)]
    struct DisplayId(u32);
    assert_eq!(&format!("{}", DisplayId(12)), "12");
    assert_eq!(&format!("{:b}", DisplayId(12)), "1100");
    assert_eq!(&format!("{:#b}", DisplayId(12)), "0b1100");
    assert_eq!(&format!("{}", Id(12)), "12");
    assert_eq!(&format!("{:b}", Id(12)), "1100");
    assert_eq!(&format!("{:#b}", Id(12)), "0b1100");
}

#[test]
fn test_add() {
    #[derive(Debug, PartialEq, Eq, id::Add)]
    struct AddId(u32);
    assert_eq!(Id(12) + Id(14), Id(26));
    assert_eq!(AddId(12) + AddId(14), AddId(26));
}

#[test]
fn test_add_inner() {
    #[derive(Debug, PartialEq, Eq, id::AddInner)]
    struct AddInnerId(u32);
    assert_eq!(Id(12) + 14, Id(26));
    assert_eq!(AddInnerId(12) + 14, AddInnerId(26));
}

#[test]
fn test_sub() {
    #[derive(Debug, PartialEq, Eq, id::Sub)]
    struct SubId(u32);
    assert_eq!(Id(14) - Id(14), Id(0));
    assert_eq!(SubId(14) - SubId(14), SubId(0));
}

#[test]
fn test_sub_inner() {
    #[derive(Debug, PartialEq, Eq, id::SubInner)]
    struct SubInnerId(u32);
    assert_eq!(Id(14) - 14, Id(0));
    assert_eq!(SubInnerId(14) - 14, SubInnerId(0));
}

#[test]
fn test_mul() {
    #[derive(Debug, PartialEq, Eq, id::Mul)]
    struct MulId(u32);
    assert_eq!(Id(12) * Id(12), Id(144));
    assert_eq!(MulId(12) * MulId(12), MulId(144));
}

#[test]
fn test_mul_inner() {
    #[derive(Debug, PartialEq, Eq, id::MulInner)]
    struct MulInnerId(u32);
    assert_eq!(Id(12) * 12, Id(144));
    assert_eq!(MulInnerId(12) * 12, MulInnerId(144));
}

#[test]
fn test_div() {
    #[derive(Debug, PartialEq, Eq, id::Div)]
    struct DivId(u32);
    assert_eq!(Id(12) / Id(3), Id(4));
    assert_eq!(DivId(12) / DivId(3), DivId(4));
}

#[test]
fn test_div_inner() {
    #[derive(Debug, PartialEq, Eq, id::DivInner)]
    struct DivId(u32);
    assert_eq!(Id(12) / 3, Id(4));
    assert_eq!(DivId(12) / 3, DivId(4));
}

#[test]
fn test_from_inner() {
    #[derive(Debug, PartialEq, Eq, id::FromInner)]
    struct FromId(u32);
    #[derive(Debug, PartialEq, Eq, id::Convert)]
    struct ConvertId(u32);
    assert_eq!(Id::from(155), Id(155));
    assert_eq!(FromId::from(155), FromId(155));
    assert_eq!(ConvertId::from(155), ConvertId(155));
}

#[test]
fn test_into_inner() {
    #[derive(Debug, PartialEq, Eq, id::IntoInner)]
    struct IntoId(u32);
    #[derive(Debug, PartialEq, Eq, id::Convert)]
    struct ConvertId(u32);
    assert_eq!(155, usize::from(Id(155)));
    assert_eq!(155, u32::from(IntoId(155)));
    assert_eq!(155, u32::from(ConvertId(155)));
}
