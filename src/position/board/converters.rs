use std::fmt;
use burn::prelude::Backend;
use burn::tensor::backend::DeviceId;
use colored::{ColoredString, Colorize};
use strum::IntoEnumIterator;
use burn::tensor::{backend, Device, Tensor};



use crate::position::coordinates::types_and_structs::Square;
use crate::{square};
use crate::position::{
    board::types_and_structs::Board,
    color::Color, 
    coordinates::types_and_structs::{Column, Row}};

impl Board {
    pub fn to_tensor <BackendType: Backend> (&self, device: &BackendType::Device) -> Tensor<BackendType, 3 > {
        let mut onehote_grid = [[[0f32; 8]; 8]; 12];
        for (&square, &piece) in &self.map {
            let shift: usize = match piece.color == self.player_to_play {
                true => 0,
                false => 1
            }; 
            let (col, row): (usize, usize) = square.into();
            onehote_grid[piece.kind as usize * 2 + shift][row-1][col-1] = 1f32;
        }
        Tensor::<BackendType, 3>::from_data(
            onehote_grid,
            device
        )
    }

    pub fn to_tensor_from_player_perspective<BackendType: Backend> (
        &self, device: &BackendType::Device, player_color: Color
    ) -> Tensor<BackendType, 3 > {
        let tensorboard = self.to_tensor(device);
        match player_color {
            Color::White => tensorboard,
            Color::Black => tensorboard.flip([0])
        }
    }
}