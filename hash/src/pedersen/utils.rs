use std::cell::RefCell;
use std::rc::Rc;
use ark_ec::CurveConfig;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::groups::curves::short_weierstrass::ProjectiveVar;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
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
thread_local! {
    static A_P0_PROJ: ProjectiveVar<StarkwareCurve, FpVar<Fp>> =
        ProjectiveVar::<StarkwareCurve, FpVar<Fp>>::new(
            FpVar::Constant(SimpleField::from_felt(P0.x)),
            FpVar::Constant(SimpleField::from_felt(P0.y)),
            FpVar::Constant(SimpleField::one()),
        );
    static A_P1_PROJ: ProjectiveVar<StarkwareCurve, FpVar<Fp>> =
        ProjectiveVar::<StarkwareCurve, FpVar<Fp>>::new(
            FpVar::Constant(SimpleField::from_felt(P1.x)),
            FpVar::Constant(SimpleField::from_felt(P1.y)),
            FpVar::Constant(SimpleField::one()),
        );
    static A_P2_PROJ: ProjectiveVar<StarkwareCurve, FpVar<Fp>> =
        ProjectiveVar::<StarkwareCurve, FpVar<Fp>>::new(
            FpVar::Constant(SimpleField::from_felt(P2.x)),
            FpVar::Constant(SimpleField::from_felt(P2.y)),
            FpVar::Constant(SimpleField::one()),
        );
    static B_P1_PROJ: ProjectiveVar<StarkwareCurve, FpVar<Fp>> =
        ProjectiveVar::<StarkwareCurve, FpVar<Fp>>::new(
            FpVar::Constant(SimpleField::from_felt(P3.x)),
            FpVar::Constant(SimpleField::from_felt(P3.y)),
            FpVar::Constant(SimpleField::one()),
        );
    static B_P2_PROJ: ProjectiveVar<StarkwareCurve, FpVar<Fp>> =
        ProjectiveVar::<StarkwareCurve, FpVar<Fp>>::new(
            FpVar::Constant(SimpleField::from_felt(P4.x)),
            FpVar::Constant(SimpleField::from_felt(P4.y)),
            FpVar::Constant(SimpleField::one()),
        );

}
// lazy_static! {
//     static ref A_P0_PROJ: ProjectiveVar<StarkwareCurve, FpVar<Fp>> = ProjectiveVar::<StarkwareCurve, FpVar<Fp>>::new(
//         FpVar::Constant(SimpleField::from_felt(P0.x)),
//         FpVar::Constant(SimpleField::from_felt(P0.y)),
//         FpVar::Constant(SimpleField::one()),
//     );
// }
impl CurveProjectiveProvider for StarkwareCurve {
    //type BaseField = Fp;
    // fn get_a_p0_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    // where
    //     Self: SWCurveConfig,
    //     Self::BaseField: PrimeField + SimpleField,
    //     <Self::BaseField as Field>::BasePrimeField: SimpleField,
    //     FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,
    //
    // {       let a = ProjectiveVar::<Self, FpVar<Self::BaseField>>::new(
    //         SimpleField::from_felt(P0.x),
    //         SimpleField::from_felt(P0.y),
    //         SimpleField::one(),
    //     );
    //     a
    // }
    fn get_a_p0_proj() -> ProjectiveVar<Self, FpVar<Self::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField,
    {
        A_P0_PROJ.with(|a| a.clone())
    }
    fn get_a_p1_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    {
        A_P1_PROJ.with(|a| a.clone())
    }

    fn get_a_p2_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    {
        A_P2_PROJ.with(|a| a.clone())
    }

    fn get_b_p1_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    {
        B_P1_PROJ.with(|a| a.clone())
    }

    fn get_b_p2_proj() -> ProjectiveVar<Self, FpVar<<Self as CurveConfig>::BaseField>>
    where
        Self: SWCurveConfig,
        Self::BaseField: PrimeField + SimpleField,
        <Self::BaseField as Field>::BasePrimeField: SimpleField,
        FpVar<Self::BaseField>: FieldVar<Self::BaseField, <Self::BaseField as Field>::BasePrimeField> + SimpleField
    {
        B_P2_PROJ.with(|a| a.clone())
    }

}


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
    CurveProjectiveProvider::get_a_p2_proj()
}

pub fn get_b_p1_proj_outer<P: SWCurveConfig + CurveProjectiveProvider>() -> ProjectiveVar::<P, FpVar<P::BaseField>>
where P::BaseField: PrimeField + SimpleField,
      <P::BaseField as Field>::BasePrimeField: SimpleField,
      FpVar<P::BaseField>: FieldVar<P::BaseField,
          <P::BaseField as Field>::BasePrimeField> + SimpleField,
{
    CurveProjectiveProvider::get_b_p1_proj()

}

pub fn get_b_p2_proj_outer<P: SWCurveConfig + CurveProjectiveProvider>() -> ProjectiveVar::<P, FpVar<P::BaseField>>
where P::BaseField: PrimeField + SimpleField,
      <P::BaseField as Field>::BasePrimeField: SimpleField,
      FpVar<P::BaseField>: FieldVar<P::BaseField,
          <P::BaseField as Field>::BasePrimeField> + SimpleField,
{
    CurveProjectiveProvider::get_b_p2_proj()
}