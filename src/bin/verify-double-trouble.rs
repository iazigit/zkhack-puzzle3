#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use double_trouble::data::puzzle_data;
use double_trouble::inner_product_argument::utils::challenge;
use double_trouble::verify;
use double_trouble::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};
use ark_ec::{group::Group};


fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (ck, [instance_and_proof_1, instance_and_proof_2]) = puzzle_data();
    let (instance1, proof1) = instance_and_proof_1;
    let (instance2, proof2) = instance_and_proof_2;

    let s1 = &proof1.response.s;
    let s2 = &proof2.response.s;
    let u1 = &proof1.response.u;
    let u2 = &proof2.response.u;
    let two = Fr::from(2);

    // Some instance parameters are reused
    // Some equations may now be simplified
    assert_eq!(instance1.comm_a, instance2.comm_a);
    assert_eq!(instance1.b, instance2.b);
    assert_eq!(proof1.commitment.comm_r.double(), proof2.commitment.comm_r);

    // Getting challenge parameters
    let chall1 = challenge(&ck, &instance1, &proof1.commitment);
    let chall2 = challenge(&ck, &instance1, &proof2.commitment);

    // Reconstructing a and alpha from equations
    let r1: Vec<Fr> = s1.iter().zip(s2).map(|(x,y)| (*x-y) / (chall1 - chall2 * two)).collect();
    let rho1 = (*u1 - u2) / (chall1 - chall2 * two);
    let a = s1.iter().zip(r1).map(|(x,r)| *x - chall1 * r).collect();
    let alpha = *u1 - chall1 * rho1;

    let tmp: Vec<Fr> = s1.iter().zip(s2).map(|(x, y)| (x.double() - y)).collect();


    assert!(verify(&ck, &instance1, &proof1));
    assert!(verify(&ck, &instance2, &proof2));

    println!("Trying..");
    let (a, comm_a_rand): (Vec<Fr>, Fr) = {
        // Your solution here!
        (a, alpha)
        
    };
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance1.comm_a
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance2.comm_a
    );
    println!("Soulved..")
}
