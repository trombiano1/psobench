extern crate nalgebra as na;
extern crate rand;
use crate::gsa;
use crate::gsa::tiled_gsa::TiledGSA;
use crate::parameters;
use crate::pso;
use crate::utils;
use gsa::gsa::GSA;
use gsa::gsa_particle::GSAParticle;
use gsa::tiled_gsa_particle::TiledGSAParticle;
use pso::particle::PSOParticle;
use pso::pso::PSO;
use std::path::PathBuf;

#[allow(dead_code)]
pub fn pso_cec17(iterations: usize, dim: usize, attempts: usize) -> Result<(), Box<dyn std::error::Error>> {
  utils::check_cec17::<PSOParticle, PSO<PSOParticle>>(
    "PSO".to_owned(),
    iterations,
    dim,
    parameters::PSO_PARAMS.clone(),
    attempts,
    PathBuf::from(format!("data/test/{}/pso", dim)),
  )?;
  Ok(())
}

#[allow(dead_code)]
pub fn gsa_cec17(iterations: usize, dim: usize, attempts: usize) -> Result<(), Box<dyn std::error::Error>> {
  utils::check_cec17::<GSAParticle, GSA<GSAParticle>>(
    "GSA".to_owned(),
    iterations,
    dim,
    parameters::GSA_PARAMS.clone(),
    attempts,
    PathBuf::from(format!("data/test/{}/gsa", dim)),
  )?;
  Ok(())
}

#[allow(dead_code)]
pub fn tiled_gsa_cec17(iterations: usize, dim: usize, attempts: usize) -> Result<(), Box<dyn std::error::Error>> {
  utils::check_cec17::<TiledGSAParticle, TiledGSA<TiledGSAParticle>>(
    "TiledGSA".to_owned(),
    iterations,
    dim,
    parameters::TILED_GSA_PARAMS.clone(),
    attempts,
    PathBuf::from(format!("data/test/{}/tiled_gsa", dim)),
  )?;
  Ok(())
}

#[allow(dead_code)]
pub fn grid_search_pso(iterations: usize, dim: usize, attempts: usize) -> Result<(), Box<dyn std::error::Error>> {
  utils::run_grid_searches::<PSOParticle, PSO<PSOParticle>>(
    "PSO".to_owned(),
    attempts,
    iterations,
    parameters::PSO_PHI_P_OPTIONS.clone(),
    parameters::PSO_PHI_G_OPTIONS.clone(),
    parameters::PSO_BASE_PARAMS.clone(),
    dim,
    PathBuf::from(format!("data/grid_search/{}/pso", dim)),
  )?;
  Ok(())
}

#[allow(dead_code)]
pub fn grid_search_gsa(iterations: usize, dim: usize, attempts: usize) -> Result<(), Box<dyn std::error::Error>> {
  utils::run_grid_searches::<GSAParticle, GSA<GSAParticle>>(
    "GSA".to_owned(),
    attempts,
    iterations,
    parameters::GSA_G0_OPTIONS.clone(),
    parameters::GSA_ALPHA_OPTIONS.clone(),
    parameters::GSA_BASE_PARAMS.clone(),
    dim,
    PathBuf::from(format!("data/grid_search/{}/gsa", dim)),
  )?;
  Ok(())
}

#[allow(dead_code)]
pub fn grid_search_tiled_gsa(iterations: usize, dim: usize, attempts: usize) -> Result<(), Box<dyn std::error::Error>> {
  utils::run_grid_searches::<TiledGSAParticle, TiledGSA<TiledGSAParticle>>(
    "GSA".to_owned(),
    attempts,
    iterations,
    parameters::GSA_G0_OPTIONS.clone(),
    parameters::GSA_ALPHA_OPTIONS.clone(),
    parameters::GSA_BASE_PARAMS.clone(),
    dim,
    PathBuf::from(format!("data/grid_search/{}/tiled_gsa", dim)),
  )?;
  Ok(())
}