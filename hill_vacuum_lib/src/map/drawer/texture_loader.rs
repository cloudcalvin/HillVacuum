//=======================================================================//
// IMPORTS
//
//=======================================================================//

use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex}
};

use arrayvec::ArrayVec;
use bevy::{
    prelude::{Assets, Image, NextState, Resource, States, Window},
    render::{
        render_asset::RenderAssetUsages,
        texture::{CompressedImageFormats, ImageSampler, ImageType}
    }
};
use bevy_egui::{egui, EguiUserTextures};
use threadpool::ThreadPool;

use super::texture::Texture;
use crate::map::{editor::state::ui::centered_window, EGUI_CYAN};

//=======================================================================//
// CONSTANTS
//
//=======================================================================//

const TEXTURES_PATH: &str = "assets/textures/";

//=======================================================================//
// ENUMS
//
//=======================================================================//

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(in crate::map) enum TextureLoadingProgress
{
    #[default]
    Initiated,
    LoadingFromFiles,
    GeneratingTextures,
    Complete
}

//=======================================================================//

#[derive(Default)]
enum LoadedImages
{
    #[default]
    Empty,
    Loading(PartialImages),
    Loaded(Vec<(String, Image)>)
}

//=======================================================================//
// TYPES
//
//=======================================================================//

type Paths = [Vec<PathBuf>; TextureLoader::THREADS_AMOUNT];

//=======================================================================//

type PartialImages = Arc<Mutex<Vec<(String, Image)>>>;

//=======================================================================//

#[must_use]
#[derive(Resource)]
pub(in crate::map) struct TextureLoader
{
    paths:               Arc<Paths>,
    images:              LoadedImages,
    textures:            Vec<(Texture, egui::TextureId)>,
    thread_pool:         ThreadPool,
    active_workers:      usize,
    cycles:              usize,
    file_reading_cycles: usize,
    total_cycles:        f32
}

impl Default for TextureLoader
{
    #[inline]
    fn default() -> Self
    {
        std::fs::create_dir_all(TEXTURES_PATH).ok();

        Self {
            paths:               Arc::new(Self::DEFAULT_PATHS),
            images:              LoadedImages::Empty,
            textures:            vec![],
            thread_pool:         ThreadPool::new(Self::THREADS_AMOUNT),
            active_workers:      0,
            cycles:              0,
            file_reading_cycles: 0,
            total_cycles:        0f32
        }
    }
}

impl TextureLoader
{
    const DEFAULT_PATHBUF_VEC: Vec<PathBuf> = vec![];
    const DEFAULT_PATHS: [Vec<PathBuf>; Self::THREADS_AMOUNT] =
        [Self::DEFAULT_PATHBUF_VEC; Self::THREADS_AMOUNT];
    const PER_FRAME_FILE_LOADS: usize = 3;
    const PER_FRAME_TEXTURE_GENERATION: usize = Self::THREADS_AMOUNT;
    const THREADS_AMOUNT: usize = 32;

    #[inline]
    #[must_use]
    pub fn loaded_textures(&mut self) -> Vec<(Texture, egui::TextureId)>
    {
        std::mem::take(&mut self.textures)
    }

    #[inline]
    fn extract_images(mut images: PartialImages) -> Vec<(String, Image)>
    {
        Arc::try_unwrap(std::mem::replace(&mut images, Arc::new(Mutex::new(vec![]))))
            .unwrap()
            .into_inner()
            .unwrap()
    }

    #[allow(clippy::cast_precision_loss)]
    #[inline]
    fn collect_paths(&mut self)
    {
        #[inline]
        fn collect_paths_recursive<P: AsRef<Path>>(
            path: P,
            paths: &mut [Vec<PathBuf>; TextureLoader::THREADS_AMOUNT],
            len: &mut usize
        )
        {
            for child_path in std::fs::read_dir(path).unwrap().map(|entry| entry.unwrap().path())
            {
                if child_path.is_dir()
                {
                    collect_paths_recursive(child_path, paths, len);
                    continue;
                }

                paths[*len % TextureLoader::THREADS_AMOUNT].push(child_path);
                *len += 1;
            }
        }

        let mut paths = Self::DEFAULT_PATHS;
        let mut textures_len = 0;
        collect_paths_recursive(TEXTURES_PATH, &mut paths, &mut textures_len);
        self.active_workers = 0;

        for vec in &paths
        {
            if vec.is_empty()
            {
                break;
            }

            self.active_workers += 1;
        }

        self.file_reading_cycles = paths[0].len().div_ceil(Self::PER_FRAME_FILE_LOADS);
        self.total_cycles = (self.file_reading_cycles +
            textures_len.div_ceil(Self::PER_FRAME_TEXTURE_GENERATION))
            as f32;
        self.cycles = 0;
        self.images = LoadedImages::Loading(Arc::new(Mutex::new(Vec::with_capacity(paths.len()))));
        self.paths = Arc::new(paths);
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[inline]
    pub fn load(
        &mut self,
        images: &mut Assets<Image>,
        user_textures: &mut EguiUserTextures,
        load_state: &mut NextState<TextureLoadingProgress>
    )
    {
        match &mut self.images
        {
            LoadedImages::Empty =>
            {
                self.collect_paths();

                if self.total_cycles == 0f32
                {
                    load_state.set(TextureLoadingProgress::Complete);
                    self.images = LoadedImages::Empty;
                }
                else
                {
                    load_state.set(TextureLoadingProgress::LoadingFromFiles);
                }
            },
            LoadedImages::Loading(vec) =>
            {
                for i in 0..self.active_workers
                {
                    let paths_len = self.paths[i].len();
                    let first = self.cycles * Self::PER_FRAME_FILE_LOADS;
                    let range = first..(first + Self::PER_FRAME_FILE_LOADS).min(paths_len);
                    let paths = self.paths.clone();
                    let images = vec.clone();

                    self.thread_pool.execute(move || {
                        let mut textures = ArrayVec::<_, { Self::THREADS_AMOUNT }>::new();

                        for j in range
                        {
                            let path = &paths[i][j];

                            let image = Image::from_buffer(
                                &std::fs::read(path).unwrap(),
                                ImageType::Extension(path.extension().unwrap().to_str().unwrap()),
                                CompressedImageFormats::all(),
                                true,
                                ImageSampler::default(),
                                RenderAssetUsages::all()
                            )
                            .unwrap();

                            textures.push((
                                path.file_stem().unwrap().to_str().unwrap().to_owned(),
                                image
                            ));
                        }

                        images.lock().unwrap().extend(textures);
                    });
                }

                self.thread_pool.join();
                self.cycles += 1;

                if self.cycles == self.file_reading_cycles
                {
                    self.images = LoadedImages::Loaded(Self::extract_images(std::mem::take(vec)));
                    load_state.set(TextureLoadingProgress::GeneratingTextures);
                }
            },
            LoadedImages::Loaded(vec) =>
            {
                for _ in 0..Self::PER_FRAME_TEXTURE_GENERATION.min(vec.len())
                {
                    let (name, image) = vec.pop().unwrap();
                    let texture = Texture::new(name, image, images);
                    let tex_id = user_textures.add_image(texture.handle());
                    self.textures.push((texture, tex_id));
                }

                self.cycles += 1;

                if vec.is_empty()
                {
                    assert!(
                        self.cycles == self.total_cycles as usize,
                        "Run cycles does not equal the projected total cycles."
                    );
                    load_state.set(TextureLoadingProgress::Complete);
                    self.images = LoadedImages::Empty;
                }
            }
        };
    }

    #[allow(clippy::cast_precision_loss)]
    #[inline]
    pub fn ui(&self, window: &Window, egui_context: &mut egui::Context)
    {
        let id = centered_window(window, "Loading textures...")
            .interactable(false)
            .default_width(400f32)
            .default_height(100f32)
            .show(egui_context, |ui| {
                ui.add(
                    egui::ProgressBar::new(self.cycles as f32 / self.total_cycles).fill(EGUI_CYAN)
                );
            })
            .unwrap()
            .response
            .layer_id;

        egui_context.move_to_top(id);
    }
}
