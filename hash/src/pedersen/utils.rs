use ark_ec::CurveConfig;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::groups::curves::short_weierstrass::ProjectiveVar;
use swiftness_field::{Fp, SimpleField};
use swiftness_utils::curve::StarkwareCurve;
use crate::pedersen::constants::{P0, P1, P2, P3, P4};

pub trait CurveProjectiveProvider: SWCurveConfig {
    //type BaseField: PrimeField + SimpleField;
    //type BasePrimeField: SimpleField;

    fn get_a_p0_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,
    ;
    fn get_a_p1_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,
    ;
    fn get_a_p2_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,
    ;
    // fn get_b_p0_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    // where
    //     Self: SWCurveConfig,
    //     Self::BaseField: PrimeField + SimpleField,
    //     <Self::BaseField as Field>::BasePrimeField: SimpleField,
    //     FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,
    // ;
    fn get_b_p1_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,
    ;
    fn get_b_p2_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,
    ;
}
//
//
impl CurveProjectiveProvider for StarkwareCurve {
    //type BaseField = Fp;
    fn get_a_p0_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,

    {       let a = ProjectiveVar::<Self, FpVar<Self::BaseField>>::new(
            SimpleField::from_felt(P0.x),
            SimpleField::from_felt(P0.y),
            SimpleField::one(),
        );
        a
    }

    fn get_a_p1_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    {
        let a = ProjectiveVar::<Self, FpVar<Self::BaseField>>::new(
            SimpleField::from_felt(P1.x),
            SimpleField::from_felt(P1.y),
            SimpleField::one(),
        );
        a
    }

    fn get_a_p2_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    {
        let a = ProjectiveVar::<Self, FpVar<Self::BaseField>>::new(
            SimpleField::from_felt(P2.x),
            SimpleField::from_felt(P2.y),
            SimpleField::one(),
        );
        a
    }

    // fn get_b_p0_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    // where
    //     Self: SWCurveConfig,
    //     Self::BaseField: PrimeField + SimpleField,
    //     <Self::BaseField as Field>::BasePrimeField: SimpleField,
    //     FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    // {
    //     let a = ProjectiveVar::<Self, FpVar<Self::BaseField>>::new(
    //         SimpleField::from_felt(P0.x),
    //         SimpleField::from_felt(P0.y),
    //         SimpleField::one(),
    //     );
    //     a
    // }

    fn get_b_p1_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    {
        let a = ProjectiveVar::<Self, FpVar<Self::BaseField>>::new(
            SimpleField::from_felt(P3.x),
            SimpleField::from_felt(P3.y),
            SimpleField::one(),
        );
        a
    }

    fn get_b_p2_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    {
        let a = ProjectiveVar::<Self, FpVar<Self::BaseField>>::new(
            SimpleField::from_felt(P4.x),
            SimpleField::from_felt(P4.y),
            SimpleField::one(),
        );
        a
    }

    // fn get_a_p1_proj() -> ProjectiveVar<Self, FpVar<Self::BaseField>> {
    //     let a = ProjectiveVar::new(
    //         SimpleField::from_felt(P1.x),
    //         SimpleField::from_felt(P1.y),
    //         SimpleField::one(),
    //     );
    //     a
    // }
    //
    // fn get_a_p2_proj() -> ProjectiveVar<Self, FpVar<Self::BaseField>> {
    //     let a = ProjectiveVar::new(
    //         SimpleField::from_felt(P2.x),
    //         SimpleField::from_felt(P2.y),
    //         SimpleField::one(),
    //     );
    //     a
    // }
}
// pub fn get_a_p0_proj() -> ProjectiveVar<StarkwareCurve, FpVar<Fp>> {
//     let a = ProjectiveVar::new(
//         SimpleField::from_felt(P0.x),
//         SimpleField::from_felt(P0.y),
//         SimpleField::one(),
//     );
//     a
// }
//
// pub fn get_a_p1_proj() -> ProjectiveVar<StarkwareCurve, FpVar<Fp>> {
//     let a = ProjectiveVar::new(
//         SimpleField::from_felt(P1.x),
//         SimpleField::from_felt(P1.y),
//         SimpleField::one(),
//     );
//     a
// }
// pub fn get_a_p2_proj() -> ProjectiveVar<StarkwareCurve, FpVar<Fp>> {
//     let a = ProjectiveVar::new(
//         SimpleField::from_felt(P2.x),
//         SimpleField::from_felt(P2.y),
//         SimpleField::one(),
//     );
//     a
// }

pub fn get_a_p0_proj_outer<P: SWCurveConfig + CurveProjectiveProvider>() -> ProjectiveVar::<P, FpVar<P::BaseField>>
where P::BaseField: PrimeField + SimpleField,
      <P::BaseField as Field>::BasePrimeField: SimpleField,
      FpVar<P::BaseField>: FieldVar<P::BaseField,
          <P::BaseField as Field>::BasePrimeField> + SimpleField,
{
    let a = CurveProjectiveProvider::get_a_p0_proj();
    a
}

pub fn get_a_p1_proj_outer<P: SWCurveConfig + CurveProjectiveProvider>() -> ProjectiveVar::<P, FpVar<P::BaseField>>
where P::BaseField: PrimeField + SimpleField,
      <P::BaseField as Field>::BasePrimeField: SimpleField,
      FpVar<P::BaseField>: FieldVar<P::BaseField,
          <P::BaseField as Field>::BasePrimeField> + SimpleField,
{
    let a = CurveProjectiveProvider::get_a_p1_proj();
    a
}

pub fn get_a_p2_proj_outer<P: SWCurveConfig + CurveProjectiveProvider>() -> ProjectiveVar::<P, FpVar<P::BaseField>>
where P::BaseField: PrimeField + SimpleField,
      <P::BaseField as Field>::BasePrimeField: SimpleField,
      FpVar<P::BaseField>: FieldVar<P::BaseField,
          <P::BaseField as Field>::BasePrimeField> + SimpleField,
{
    let a = CurveProjectiveProvider::get_a_p2_proj();
    a
}

pub fn get_b_p1_proj_outer<P: SWCurveConfig + CurveProjectiveProvider>() -> ProjectiveVar::<P, FpVar<P::BaseField>>
where P::BaseField: PrimeField + SimpleField,
      <P::BaseField as Field>::BasePrimeField: SimpleField,
      FpVar<P::BaseField>: FieldVar<P::BaseField,
          <P::BaseField as Field>::BasePrimeField> + SimpleField,
{
    let a = CurveProjectiveProvider::get_b_p1_proj();
    a
}

pub fn get_b_p2_proj_outer<P: SWCurveConfig + CurveProjectiveProvider>() -> ProjectiveVar::<P, FpVar<P::BaseField>>
where P::BaseField: PrimeField + SimpleField,
      <P::BaseField as Field>::BasePrimeField: SimpleField,
      FpVar<P::BaseField>: FieldVar<P::BaseField,
          <P::BaseField as Field>::BasePrimeField> + SimpleField,
{
    let a = CurveProjectiveProvider::get_b_p2_proj();
    a
}