mod id;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub use self::id::ID;
use super::object::Object;

#[derive(Clone)]
pub struct Scene<'a> {
    objects: HashMap<ID, Rc<RefCell<dyn Object + 'a>>>,
    subscenes: HashMap<ID, Scene<'a>>
}

impl<'a> Scene<'a> {
    /// Creates empty Scene
    /// ```
    /// # use rayimg::Scene;
    /// let test_scene = Scene::new();
    /// assert!(test_scene.object_count() == 0 && test_scene.subscene_count() == 0)
    /// ```
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            subscenes: HashMap::new()
        }
    }

    /// Adds object to scene
    /// ```
    /// # use rayimg::{Scene, shapes::Sphere, Vec3};
    /// let mut test_scene = Scene::new();
    /// test_scene.add_object(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0));
    /// assert_eq!(test_scene.object_count(), 1);
    /// ```
    pub fn add_object(&mut self, object: impl Object + 'a) -> ID {
        let object_id = ID::new();
        // self.objects.insert(object_id, Box::new(object));
        self.objects.insert(object_id, Rc::new(RefCell::new(object)));
        object_id
    }

    /// Removes object from scene
    /// ```
    /// # use rayimg::{Scene, shapes::Sphere, Vec3};
    /// let mut test_scene = Scene::new();
    /// let unit_sphere_id = test_scene.add_object(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0));
    /// assert_eq!(test_scene.object_count(), 1);
    /// test_scene.remove_object(&unit_sphere_id);
    /// assert_eq!(test_scene.object_count(), 0);
    /// ```
    pub fn remove_object(&mut self, id: &ID) -> Option<Rc<RefCell<dyn Object + 'a>>> {
        self.objects.remove(id)
    }

    pub fn get_object(&self, id: &ID) -> Option<&dyn Object> {
        unimplemented!();
    }

    pub fn get_object_mut(&mut self, id: &ID) -> Option<&mut dyn Object> {
        unimplemented!();
    }

    /// Returns count of objects
    /// ```
    /// # use rayimg::{Scene, shapes::Sphere, Vec3};
    /// let mut test_scene = Scene::new();
    /// let unit_sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
    /// test_scene.add_object(unit_sphere.clone());
    /// test_scene.add_object(unit_sphere.clone());
    /// test_scene.add_object(unit_sphere.clone());
    /// assert_eq!(test_scene.object_count(), 3);
    /// ```
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Adds subscene to scene
    /// ```
    /// # use rayimg::Scene;
    /// let (mut main_scene, test_scene) = (Scene::new(), Scene::new());
    /// main_scene.add_subscene(test_scene);
    /// assert_eq!(main_scene.subscene_count(), 1);
    /// ```
    pub fn add_subscene(&mut self, scene: Scene<'a>) -> ID {
        let scene_id = ID::new();
        self.subscenes.insert(scene_id, scene);
        scene_id
    }

    /// Removes subscene from scene
    /// ```
    /// # use rayimg::Scene;
    /// let (mut main_scene, test_scene) = (Scene::new(), Scene::new());
    /// let test_scene_id = main_scene.add_subscene(test_scene);
    /// assert_eq!(main_scene.subscene_count(), 1);
    /// main_scene.remove_subscene(&test_scene_id);
    /// assert_eq!(main_scene.subscene_count(), 0);
    /// ```
    pub fn remove_subscene(&mut self, id: &ID) -> Option<Scene<'a>> {
        self.subscenes.remove(id)
    }

    pub fn get_subscene(&self, id: &ID) -> Option<&Scene> {
        unimplemented!();
    }

    pub fn get_subscene_mut(&mut self, id: &ID) -> Option<&'a mut Scene> {
        unimplemented!();
    }

    /// Returns count of subscenes
    /// ```
    /// # use rayimg::Scene;
    /// let (mut main_scene, test_scene) = (Scene::new(), Scene::new());
    /// main_scene.add_subscene(test_scene.clone());
    /// main_scene.add_subscene(test_scene.clone());
    /// main_scene.add_subscene(test_scene.clone());
    /// main_scene.add_subscene(test_scene.clone());
    /// assert_eq!(main_scene.subscene_count(), 4);
    /// ```
    pub fn subscene_count(&self) -> usize {
        self.subscenes.len()
    }
}
