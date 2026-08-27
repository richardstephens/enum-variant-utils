use evutils::step_count::{Step, StepCount};

#[derive(StepCount)]
pub enum Onboarding {
    CreateAccount,
    VerifyEmail,
    AddPaymentDetails(String),
    Done { at: u64 },
}

#[derive(StepCount)]
pub enum SingleStep {
    Only,
}

#[test]
fn first_and_last_steps() {
    assert_eq!("1 / 4", Onboarding::CreateAccount.step().to_string());
    assert_eq!("4 / 4", Onboarding::Done { at: 0 }.step().to_string());
}

#[test]
fn middle_steps() {
    assert_eq!("2 / 4", format!("{}", Onboarding::VerifyEmail.step()));
    assert_eq!(
        "3 / 4",
        format!(
            "{}",
            Onboarding::AddPaymentDetails("card".to_string()).step()
        )
    );
}

#[test]
fn step_and_total_accessors() {
    let step = Onboarding::VerifyEmail.step();
    assert_eq!(2, step.step());
    assert_eq!(4, step.total());
}

#[test]
fn single_variant_enum() {
    assert_eq!("1 / 1", SingleStep::Only.step().to_string());
}

#[test]
#[allow(
    clippy::clone_on_copy,
    reason = "deliberately exercising the Clone impl"
)]
fn step_is_copy_and_clone() {
    let step = Onboarding::CreateAccount.step();
    let copied = step;
    let cloned = step.clone();
    assert_eq!(step, copied);
    assert_eq!(step, cloned);
    // `step` is still usable after being copied out of.
    assert_eq!("1 / 4", step.to_string());
}

#[test]
fn steps_can_be_compared_and_constructed() {
    assert_eq!(Step::new(1, 4), Onboarding::CreateAccount.step());
    assert_ne!(Step::new(1, 4), Onboarding::VerifyEmail.step());
}
