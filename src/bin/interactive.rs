
#![feature(let_chains)]
#![feature(async_fn_in_trait)]

use std::{sync::{RwLock, Arc}, thread::JoinHandle, io::stdin, f32::consts::E};

use checkers::{Board, CellPos, Color, MoveDir, PieceMove, DIRS, cell};
use ellipsoid::prelude::{*, winit::event::{ElementState, MouseButton}};
use strum::{Display, EnumIter};

type Txts = CheckersTextures;

struct Checkers {
    graphics: Graphics<Txts>,
    game_data: Arc<RwLock<(Color, Board)>>,
    receiver_handle: JoinHandle<()>,
    selected_square: Option<CellPos>,
    mouse_pos: Vec2,
}

#[derive(Debug, Clone, Copy, EnumIter, Display, Default)]
#[strum(serialize_all = "snake_case")]
enum CheckersTextures {
    #[default]
    White,
}

impl Into<u32> for CheckersTextures {
    fn into(self) -> u32 {
        self as u32
    }
}

impl Textures for CheckersTextures {}

impl App<Txts> for Checkers {
    async fn new(window: winit::window::Window) -> Self {
        let graphics = Graphics::<Txts>::new(window).await;
        let game_data = Arc::new(RwLock::new((Color::White, Board::new())));
        let gdc = game_data.clone();
        let receiver_handle = std::thread::spawn(|| {
            receiver(gdc);
        });
        Self {
            graphics,
            game_data,
            receiver_handle,
            selected_square: None,
            mouse_pos: Vec2::ZERO,
        }
    }

    fn update(&mut self, dt: f32) {
    }

    fn draw(&mut self) {
        let (color, mut board) = self.game_data.read().unwrap().clone();
        if let Some(cp) = self.selected_square && board.turn == color {
            egui::Window::new("Move").show(&self.graphics.egui_platform.context(), |ui| {
                let old_board = board.clone();
                let valid_dirs = DIRS.into_iter().filter(|&dir| {
                    if board.make_move(PieceMove {pos: cp, dir}) {
                        board = old_board.clone();
                        true
                    }
                    else {
                        false
                    }
                }).collect::<Vec<_>>();
                if valid_dirs.is_empty() {
                    ui.label("No available moves.");
                }
                else {
                    for dir in valid_dirs {
                        if ui.button(format!("{}", dir)).clicked() {
                            let pm = PieceMove {pos: cp, dir};
                            assert!(self.game_data.write().unwrap().1.make_move(pm));
                            println!("{}", pm);
                        }
                    }
                }
            });
        }


        let board_gt = GTransform::from_translation(Vec2::NEG_ONE).inflate(2.);

        for i in 0..8 {
            for j in 0..8 {
                let mut cell_shape = Shape::from_square().set_color(if (i+j)%2 == 0 { ellipsoid::Color::from_hex(0x964d37) } else {ellipsoid::Color::from_hex(0xdad9b5)}).set_z(0.9);
                let cell_gt = GTransform::from_translation(Vec2::new(j as f32*1./8., i as f32*1./8.)).inflate(1./8.);

                if let Some(cp) = self.selected_square {
                    if cp == cell(i, j) {
                        cell_shape = cell_shape.set_color(ellipsoid::Color::RED);
                    }
                }

                self.graphics.add_geometry(cell_shape.apply(cell_gt).apply(board_gt).into());
                if let Some(piece) = board[cell(i, j)].piece {
                    let get_col = |col| if col == Color::White { ellipsoid::Color::WHITE } else { ellipsoid::Color::BLACK };

                    let piece_shape = Shape::from_circle(20).set_color(get_col(piece.color)).apply(GTransform::from_inflation(0.3)).set_z(0.7);
                    let piece_ol_shape = Shape::from_circle(20).set_color(get_col(-piece.color)).apply(GTransform::from_inflation(0.34)).set_z(0.8);

                    let cell_gt = cell_gt.translate(Vec2::ONE * 0.5);

                    self.graphics.add_geometry(piece_ol_shape.clone().apply(cell_gt).apply(board_gt).into());
                    self.graphics.add_geometry(piece_shape.clone().apply(cell_gt).apply(board_gt).into());

                    if piece.king {
                        self.graphics.add_geometry(piece_ol_shape.apply(cell_gt).set_z(0.6).apply(board_gt).apply(GTransform::from_translation(Vec2::ONE*0.01)).into());
                        self.graphics.add_geometry(piece_shape.apply(cell_gt).set_z(0.5).apply(board_gt).apply(GTransform::from_translation(Vec2::ONE*0.01)).into());
                    }
                }

            }
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        // on mosue move update mouse pos
        // on mouse click select square

        match event {
            WindowEvent::CursorMoved {position, ..} => {
                let window_size = self.graphics.window().inner_size();
                let window_size = vec2(window_size.width as f32, window_size.height as f32);
                let position = vec2(position.x as f32, position.y as f32) / window_size;
                self.mouse_pos = vec2(position.x, 1. - position.y);
            }
            WindowEvent::MouseInput {state, button, ..} => {
                if *state == ElementState::Pressed && *button == MouseButton::Right {
                    let row = (self.mouse_pos.y * 8.) as usize;                    
                    let col = (self.mouse_pos.x * 8.) as usize;

                    if row < 8 && col < 8 {
                        self.selected_square = Some(cell(row, col));
                    }
                }
            }
            _ => {}
        }
        false

    }

    fn graphics_mut(&mut self) -> &mut Graphics<Txts> {
        &mut self.graphics
    }

    fn graphics(&self) -> &Graphics<Txts> {
        &self.graphics
    }
}

fn main() {
    async_std::task::block_on(start());
}

fn receiver(game_data: Arc<RwLock<(Color, Board)>>) {
    let stdin = stdin();

    loop {
        let mut inp = String::new();
        stdin.read_line(&mut inp).unwrap();
        inp = inp.trim().into();
        if inp == "white" || inp == "black" {
            game_data.write().unwrap().0 = Color::from_str(&inp);
            continue;
        }
        if inp == "exit" {
            break;
        }
        let must_jump = inp.split_whitespace().map(|pstr| {
            CellPos::from_str(pstr)
        }).collect::<Vec<_>>();

        // eprintln!("Reading board from stdin...");
        let board = Board::from_stdin(&stdin);
        let mut game_data = game_data.write().unwrap();
        game_data.1 = board;
        game_data.1.turn = game_data.0;
        game_data.1.must_jump = must_jump;
    }
}

pub async fn start() {
    ellipsoid::run::<Txts, Checkers>().await;
}